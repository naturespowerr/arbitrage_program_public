use crate::arbitrage_engine::{BasePool, LiquidityType, SwapResult};
use crate::commons::get_ata_balance;
use crate::dex::pumpswap::*;
use crate::error::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

pub struct PumpswapPool<'b, 'info> {
    pub is_source_quote: bool,
    pub protocol_fee_basis_points: u64,
    pub lp_fee_basis_points: u64,
    pub coin_creator_fee_basis_points: u64,
    pub base_reserve: u64,
    pub quote_reserve: u64,

    pub accounts: &'b PumpswapAccounts<'info>,
    pub common: &'b CommonAccounts<'info>,
}

impl<'b, 'info> PumpswapPool<'b, 'info> {
    pub fn new(
        common: &'b CommonAccounts<'info>,
        accounts: &'b PumpswapAccounts<'info>,
    ) -> Result<Self> {
        // Deserialization 232 CU
        let pool_bytes = accounts.pool.try_borrow_data()?;
        let (base_mint_pubkey, quote_mint_pubkey) = PfAmmPool::deserialize_mints(&pool_bytes)?;

        // let global_config_bytes = accounts.global_config.try_borrow_data()?;
        // let global_config = GlobalConfig::deserialize_data(&global_config_bytes)?;

        let fee_config_bytes = accounts.fee_config.try_borrow_data()?;
        let fee_config = FeeConfig::deserialize_data(&fee_config_bytes)?;

        // Init checks 45 CU
        let source_in_pool = common.user_source_token_mint.key == &base_mint_pubkey
            || common.user_source_token_mint.key == &quote_mint_pubkey;
        let intermediate_in_pool = common.user_intermediate_token_mint.key == &base_mint_pubkey
            || common.user_intermediate_token_mint.key == &quote_mint_pubkey;

        require!(
            source_in_pool && intermediate_in_pool,
            ErrorCode::TokenMintMismatch
        );

        require!(
            common.user_source_token_mint.key != common.user_intermediate_token_mint.key,
            ErrorCode::TokenMintMismatch
        );

        // Определяем, является ли source_token_account quote токеном
        let is_source_quote: bool = common.user_source_token_mint.key == &quote_mint_pubkey;

        let (base_reserve, quote_reserve) = (
            get_ata_balance(&accounts.base_token_account)?,
            get_ata_balance(&accounts.quote_token_account)?,
        );

        // Получаем supply базового токена
        let base_mint_info = Mint::try_deserialize(&mut accounts.base_mint.data.borrow().as_ref())?;
        let base_mint_supply = base_mint_info.supply;

        // Рассчитываем комиссии с использованием новой логики
        let fees = compute_fees_bps(fee_config, base_mint_supply, base_reserve, quote_reserve)?;

        // msg!(
        //     "base_reserve={}, quote_reserve={}, fees: lp={}, protocol={}, creator={}",
        //     base_reserve,
        //     quote_reserve,
        //     fees.lp_fee_bps,
        //     fees.protocol_fee_bps,
        //     fees.creator_fee_bps
        // );

        Ok(Self {
            is_source_quote,
            protocol_fee_basis_points: fees.protocol_fee_bps,
            lp_fee_basis_points: fees.lp_fee_bps,
            coin_creator_fee_basis_points: fees.creator_fee_bps,
            base_reserve,
            quote_reserve,

            accounts,
            common,
        })
    }

    // Вспомогательный метод для нормализации направления свопа
    pub fn normalize_swap_direction(&self, swap_for_quote: bool) -> bool {
        if self.is_source_quote {
            swap_for_quote
        } else {
            !swap_for_quote
        }
    }

    pub fn normalize_price(&self, price: f64) -> f64 {
        if self.is_source_quote {
            price
        } else {
            1.0 / price
        }
    }
}

impl<'b, 'info> BasePool for PumpswapPool<'b, 'info> {
    fn get_fee_rate_f64(&self) -> f64 {
        let fee_denominator = 10000;
        let fee_multiplier = fee_denominator
            - self.protocol_fee_basis_points
            - self.lp_fee_basis_points
            - self.coin_creator_fee_basis_points;

        1.0 - ((fee_multiplier as f64) / (fee_denominator as f64))
    }

    fn get_price(&self) -> Result<f64> {
        Ok(self.normalize_price(self.quote_reserve as f64 / self.base_reserve as f64))
    }

    fn get_amount_out(&self, amount_in: u64, swap_for_quote: bool) -> Result<SwapResult> {
        let normalized_swap_for_quote = self.normalize_swap_direction(swap_for_quote);

        let result = get_amount_out(
            self.base_reserve,
            self.quote_reserve,
            amount_in,
            &[
                self.coin_creator_fee_basis_points,
                self.lp_fee_basis_points,
                self.protocol_fee_basis_points,
            ],
            normalized_swap_for_quote,
        );

        let res = SwapResult {
            amount_out: result.amount_out,
            consumed_in_amount: amount_in,
            is_fully_filled: true,
            new_price: self.normalize_price(result.new_price),
        };

        Ok(res)
    }

    fn get_amount_in(&self, amount_out: u64, swap_for_quote: bool) -> Result<SwapResult> {
        let normalized_swap_for_quote = self.normalize_swap_direction(swap_for_quote);

        let result = get_amount_in(
            self.base_reserve,
            self.quote_reserve,
            amount_out,
            &[
                self.coin_creator_fee_basis_points,
                self.lp_fee_basis_points,
                self.protocol_fee_basis_points,
            ],
            normalized_swap_for_quote,
        );

        Ok(SwapResult {
            amount_out,
            consumed_in_amount: result.amount_in,
            is_fully_filled: true,
            new_price: self.normalize_price(result.new_price),
        })
    }

    fn get_pool_type(&self) -> Result<LiquidityType> {
        Ok(LiquidityType::Constant)
    }

    fn get_base_quote_product(&self) -> Result<u128> {
        Ok(self.base_reserve as u128 * self.quote_reserve as u128)
    }

    fn get_max_amount(&self, _price_delta: f64, _base_to_quote: bool) -> Result<u64> {
        Ok(u64::MAX)
    }

    fn swap(
        &self,
        amount_in: u64,
        min_amount_out: u64,
        source_to_intermediate: bool,
    ) -> Result<()> {
        let (user_base_token_account, user_quote_token_account) = if self.is_source_quote {
            (
                &self.common.user_intermediate_token_account,
                &self.common.user_source_token_account,
            )
        } else {
            (
                &self.common.user_source_token_account,
                &self.common.user_intermediate_token_account,
            )
        };

        if (source_to_intermediate && self.is_source_quote)
            || (!source_to_intermediate && !self.is_source_quote)
        {
            // Вызываем buy
            pumpswap_buy(
                &self.accounts.program_id,
                &self.accounts.pool,
                &self.common.user,
                &self.accounts.global_config,
                &self.accounts.base_mint,
                &self.accounts.quote_mint,
                user_base_token_account,
                user_quote_token_account,
                &self.accounts.base_token_account,
                &self.accounts.quote_token_account,
                &self.accounts.protocol_fee_recipient,
                &self.accounts.protocol_fee_recipient_token_account,
                &self.common.token_program,
                &self.common.token_program,
                &self.common.system_program,
                &self.common.associated_token_program,
                &self.accounts.event_authority,
                &self.accounts.creator_vault,
                &self.accounts.creator,
                &self.accounts.global_volume_accumulator,
                &self.accounts.user_volume_accumulator,
                &self.accounts.fee_config,
                &self.accounts.fee_program,
                min_amount_out, // base_amount_out
                amount_in,      // max_quote_amount_in
            )
        } else {
            // Вызываем sell
            pumpswap_sell(
                &self.accounts.program_id,
                &self.accounts.pool,
                &self.common.user,
                &self.accounts.global_config,
                &self.accounts.base_mint,
                &self.accounts.quote_mint,
                user_base_token_account,
                user_quote_token_account,
                &self.accounts.base_token_account,
                &self.accounts.quote_token_account,
                &self.accounts.protocol_fee_recipient,
                &self.accounts.protocol_fee_recipient_token_account,
                &self.common.token_program,
                &self.common.token_program,
                &self.common.system_program,
                &self.common.associated_token_program,
                &self.accounts.event_authority,
                &self.accounts.creator_vault,
                &self.accounts.creator,
                &self.accounts.fee_config,
                &self.accounts.fee_program,
                amount_in,      // base_amount_in
                min_amount_out, // min_quote_amount_out
            )
        }
    }
}
