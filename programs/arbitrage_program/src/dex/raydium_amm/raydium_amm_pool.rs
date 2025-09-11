use crate::arbitrage_engine::{BasePool, LiquidityType, SwapResult};
use crate::commons::get_ata_balance;
use crate::dex::raydium_amm::*;
use crate::error::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

pub struct RaydiumAmmPool<'b, 'info> {
    pub is_source_quote: bool,
    pub swap_fee: u64,
    pub base_reserve: u64,
    pub quote_reserve: u64,
    pub base_reserve_without_take_pnl: u64,
    pub quote_reserve_without_take_pnl: u64,

    pub accounts: &'b RaydiumAmmAccounts<'info>,
    pub common: &'b CommonAccounts<'info>,
}

impl<'b, 'info> RaydiumAmmPool<'b, 'info> {
    pub fn new(
        common: &'b CommonAccounts<'info>,
        accounts: &'b RaydiumAmmAccounts<'info>,
    ) -> Result<Self> {
        // Десериализуем данные из аккаунта AMM для проверки
        let amm_data = accounts.amm_info.try_borrow_data()?;
        let (coin_mint, pc_mint) = AmmInfo::deserialize_mints(&amm_data)?;
        let (swap_fee_numerator, _) = AmmInfo::deserialize_swap_fees(&amm_data)?;
        let (need_take_pnl_coin, need_take_pnl_pc) = AmmInfo::deserialize_need_take_pnl(&amm_data)?;

        // Проверяем, что оба токена присутствуют в пуле
        let source_in_pool = common.user_source_token_mint.key == &coin_mint
            || common.user_source_token_mint.key == &pc_mint;
        let intermediate_in_pool = common.user_intermediate_token_mint.key == &coin_mint
            || common.user_intermediate_token_mint.key == &pc_mint;

        require!(
            source_in_pool && intermediate_in_pool,
            ErrorCode::TokenMintMismatch
        );

        // Проверяем, что токены разные
        require!(
            common.user_source_token_mint.key != common.user_intermediate_token_mint.key,
            ErrorCode::TokenMintMismatch
        );

        // Определяем, является ли source_token_account quote токеном (pc)
        let is_source_quote: bool = common.user_source_token_mint.key == &pc_mint;

        let (base_reserve, quote_reserve) = (
            get_ata_balance(&accounts.coin_vault)?,
            get_ata_balance(&accounts.pc_vault)?,
        );

        let (base_reserve_without_take_pnl, quote_reserve_without_take_pnl) =
            calc_total_without_take_pnl_no_orderbook(
                base_reserve,
                quote_reserve,
                need_take_pnl_pc,
                need_take_pnl_coin,
            );

        let swap_fee = swap_fee_numerator;

        // msg!(
        //     "base_reserve_without_take_pnl={}, quote_reserve_without_take_pnl={}",
        //     base_reserve_without_take_pnl,
        //     quote_reserve_without_take_pnl,
        // );

        Ok(Self {
            is_source_quote,
            swap_fee,
            base_reserve,
            quote_reserve,
            base_reserve_without_take_pnl,
            quote_reserve_without_take_pnl,
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

impl<'b, 'info> BasePool for RaydiumAmmPool<'b, 'info> {
    fn get_fee_rate_f64(&self) -> f64 {
        let fee_denominator = 10000.0;
        let fee_multiplier = fee_denominator - (self.swap_fee as f64);

        1.0 - ((fee_multiplier as f64) / (fee_denominator as f64))
    }

    fn get_price(&self) -> Result<f64> {
        Ok(self.normalize_price(
            self.quote_reserve_without_take_pnl as f64 / self.base_reserve_without_take_pnl as f64,
        ))
    }
    fn get_amount_out(&self, amount_in: u64, swap_for_quote: bool) -> Result<SwapResult> {
        let normalized_swap_for_quote = self.normalize_swap_direction(swap_for_quote);

        let result = get_amount_out(
            self.base_reserve_without_take_pnl,
            self.quote_reserve_without_take_pnl,
            amount_in,
            self.swap_fee,
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
            self.base_reserve_without_take_pnl,
            self.quote_reserve_without_take_pnl,
            amount_out,
            self.swap_fee,
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
        Ok(
            self.base_reserve_without_take_pnl as u128
                * self.quote_reserve_without_take_pnl as u128,
        )
    }

    fn get_max_amount(&self, _price_delta: f64, _swap_for_quote: bool) -> Result<u64> {
        Ok(u64::MAX)
    }

    fn swap(
        &self,
        amount_in: u64,
        min_amount_out: u64,
        source_to_intermediate: bool,
    ) -> Result<()> {
        // Определяем, какие токен-аккаунты использовать в зависимости от направления свапа
        let (user_token_in, user_token_out) = if source_to_intermediate {
            (
                &self.common.user_source_token_account,
                &self.common.user_intermediate_token_account,
            )
        } else {
            (
                &self.common.user_intermediate_token_account,
                &self.common.user_source_token_account,
            )
        };

        // Вызываем функцию свапа
        raydium_amm_v4_swap(
            &self.common.token_program,
            &self.accounts.program_id,
            &self.accounts.amm_info,
            &self.accounts.amm_authority,
            &self.accounts.amm_info, // amm_open_orders
            &self.accounts.coin_vault,
            &self.accounts.pc_vault,
            &self.accounts.amm_info, // market_program
            &self.accounts.amm_info, // market
            &self.accounts.amm_info, // market_bids
            &self.accounts.amm_info, // market_asks
            &self.accounts.amm_info, // market_event_queue
            &self.accounts.amm_info, // market_coin_vault_account
            &self.accounts.amm_info, // market_pc_vault
            &self.accounts.amm_info, // market_vault_signer
            user_token_in,
            user_token_out,
            &self.common.user,
            amount_in,
            min_amount_out,
        )
    }
}
