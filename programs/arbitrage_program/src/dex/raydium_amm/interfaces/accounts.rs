use crate::{dex::raydium_amm::*, error::ErrorCode};
use anchor_lang::prelude::{borsh::BorshDeserialize, *};

#[derive(Debug, Clone, Copy, BorshDeserialize)]
pub struct StateData {
    /// delay to take pnl coin
    pub need_take_pnl_coin: u64,
    /// delay to take pnl pc
    pub need_take_pnl_pc: u64,
    /// total pnl pc
    pub total_pnl_pc: u64,
    /// total pnl coin
    pub total_pnl_coin: u64,
    /// ido pool open time
    pub pool_open_time: u64,
    /// padding for future updates
    pub padding: [u64; 2],
    /// switch from orderbookonly to init
    pub orderbook_to_init_time: u64,

    /// swap coin in amount
    pub swap_coin_in_amount: u128,
    /// swap pc out amount
    pub swap_pc_out_amount: u128,
    /// charge pc as swap fee while swap pc to coin
    pub swap_acc_pc_fee: u64,

    /// swap pc in amount
    pub swap_pc_in_amount: u128,
    /// swap coin out amount
    pub swap_coin_out_amount: u128,
    /// charge coin as swap fee while swap coin to pc
    pub swap_acc_coin_fee: u64,
}

#[derive(Debug, Clone, Copy, BorshDeserialize)]
pub struct AmmInfo {
    /// Initialized status.
    pub status: u64,
    /// Nonce used in program address.
    /// The program address is created deterministically with the nonce,
    /// amm program id, and amm account pubkey.  This program address has
    /// authority over the amm's token coin account, token pc account, and pool
    /// token mint.
    pub nonce: u64,
    /// max order count
    pub order_num: u64,
    /// within this range, 5 => 5% range
    pub depth: u64,
    /// coin decimal
    pub coin_decimals: u64,
    /// pc decimal
    pub pc_decimals: u64,
    /// amm machine state
    pub state: u64,
    /// amm reset_flag
    pub reset_flag: u64,
    /// min size 1->0.000001
    pub min_size: u64,
    /// vol_max_cut_ratio numerator, sys_decimal_value as denominator
    pub vol_max_cut_ratio: u64,
    /// amount wave numerator, sys_decimal_value as denominator
    pub amount_wave: u64,
    /// coinLotSize 1 -> 0.000001
    pub coin_lot_size: u64,
    /// pcLotSize 1 -> 0.000001
    pub pc_lot_size: u64,
    /// min_cur_price: (2 * amm.order_num * amm.pc_lot_size) * max_price_multiplier
    pub min_price_multiplier: u64,
    /// max_cur_price: (2 * amm.order_num * amm.pc_lot_size) * max_price_multiplier
    pub max_price_multiplier: u64,
    /// system decimal value, used to normalize the value of coin and pc amount
    pub sys_decimal_value: u64,
    /// All fee information
    pub fees: Fees,
    /// Statistical data
    pub state_data: StateData,
    /// Coin vault
    pub coin_vault: Pubkey,
    /// Pc vault
    pub pc_vault: Pubkey,
    /// Coin vault mint
    pub coin_vault_mint: Pubkey,
    /// Pc vault mint
    pub pc_vault_mint: Pubkey,
    /// lp mint
    pub lp_mint: Pubkey,
    /// open_orders key
    pub open_orders: Pubkey,
    /// market key
    pub market: Pubkey,
    /// market program key
    pub market_program: Pubkey,
    /// target_orders key
    pub target_orders: Pubkey,
    /// padding
    pub padding1: [u64; 8],
    /// amm owner key
    pub amm_owner: Pubkey,
    /// pool lp amount
    pub lp_amount: u64,
    /// client order id
    pub client_order_id: u64,
    /// recent epoch
    pub recent_epoch: u64,
    /// padding
    pub padding2: u64,
}

impl AmmInfo {
    pub fn deserialize_data(data: &[u8]) -> Result<AmmInfo> {
        let result: AmmInfo =
            AmmInfo::try_from_slice(data).map_err(|_| error!(ErrorCode::InvalidAccount))?;

        Ok(result)
    }

    pub fn deserialize_mints(data: &[u8]) -> Result<(Pubkey, Pubkey)> {
        // Проверка, чтобы хватило данных до минтов
        let coin_vault_mint_offset = 0    // дискриминатор
            + 8 * 16                      // 16 полей по 8 байт (u64)
            + 8 * 8                       // Fees: 8 полей по 8 байт
            + 8 * 10 + 4 * 16             // StateData: 10 полей по 8 байт и 4 поля по 16
            + 32 * 2; // 2 pubkey

        let pc_vault_mint_offset = coin_vault_mint_offset + 32;

        if data.len() < pc_vault_mint_offset + 32 {
            return Err(ErrorCode::InvalidAccount.into());
        }

        let coin_vault_mint = Pubkey::new_from_array(
            data[coin_vault_mint_offset..coin_vault_mint_offset + 32]
                .try_into()
                .map_err(|_| ErrorCode::InvalidAccount)?,
        );

        let pc_vault_mint = Pubkey::new_from_array(
            data[pc_vault_mint_offset..pc_vault_mint_offset + 32]
                .try_into()
                .map_err(|_| ErrorCode::InvalidAccount)?,
        );

        Ok((coin_vault_mint, pc_vault_mint))
    }

    pub fn deserialize_swap_fees(data: &[u8]) -> Result<(u64, u64)> {
        if data.len() < 8 {
            return Err(ErrorCode::InvalidAccount.into());
        }

        // offset к Fees
        let swap_fee_numerator_offset = 0    // дискриминатор
            + 8 * 16                         // 16 полей по 8 байт (до Fees)
            + 8 * 6; // 6 полей внутри Fees

        let swap_fee_denominator_offset = swap_fee_numerator_offset + 8; // сразу после numerator

        if data.len() < swap_fee_denominator_offset + 8 {
            return Err(ErrorCode::InvalidAccount.into());
        }

        let swap_fee_numerator = u64::from_le_bytes(
            data[swap_fee_numerator_offset..swap_fee_numerator_offset + 8]
                .try_into()
                .map_err(|_| ErrorCode::InvalidAccount)?,
        );

        let swap_fee_denominator = u64::from_le_bytes(
            data[swap_fee_denominator_offset..swap_fee_denominator_offset + 8]
                .try_into()
                .map_err(|_| ErrorCode::InvalidAccount)?,
        );

        Ok((swap_fee_numerator, swap_fee_denominator))
    }

    pub fn deserialize_need_take_pnl(data: &[u8]) -> Result<(u64, u64)> {
        if data.len() < 8 {
            return Err(ErrorCode::InvalidAccount.into());
        }

        // offset к StateData
        let state_data_offset = 0    // дискриминатор
            + 8 * 16                 // 16 полей по 8 байт (до Fees)
            + 8 * 8; // Fees: 8 полей по 8 байт

        let need_take_pnl_coin_offset = state_data_offset;
        let need_take_pnl_pc_offset = state_data_offset + 8;

        if data.len() < need_take_pnl_pc_offset + 8 {
            return Err(ErrorCode::InvalidAccount.into());
        }

        let need_take_pnl_coin = u64::from_le_bytes(
            data[need_take_pnl_coin_offset..need_take_pnl_coin_offset + 8]
                .try_into()
                .map_err(|_| ErrorCode::InvalidAccount)?,
        );

        let need_take_pnl_pc = u64::from_le_bytes(
            data[need_take_pnl_pc_offset..need_take_pnl_pc_offset + 8]
                .try_into()
                .map_err(|_| ErrorCode::InvalidAccount)?,
        );

        Ok((need_take_pnl_coin, need_take_pnl_pc))
    }
}
