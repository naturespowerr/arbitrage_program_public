use crate::{error::ErrorCode, Pubkey, Result};
use anchor_lang::prelude::{borsh::BorshDeserialize, *};

pub const GLOBAL_CONFIG_ACCOUNT_DISCM: [u8; 8] = [149, 8, 156, 202, 160, 252, 176, 217];
#[derive(Debug, Clone, Copy, BorshDeserialize)]
pub struct GlobalConfig {
    /// The admin pubkey
    pub admin: Pubkey,
    /// The lp fee in basis points (0.01%)
    pub lp_fee_basis_points: u64,
    /// The protocol fee in basis points (0.01%)
    pub protocol_fee_basis_points: u64,
    /// Flags to disable certain functionality
    /// bit 0 - Disable create pool
    /// bit 1 - Disable deposit
    /// bit 2 - Disable withdraw
    /// bit 3 - Disable buy
    /// bit 4 - Disable sell
    pub disable_flags: u8,
    /// Addresses of the protocol fee recipients
    pub protocol_fee_recipients: [Pubkey; 8],
}

impl GlobalConfig {
    pub fn deserialize_data(data: &[u8]) -> Result<GlobalConfig> {
        pub const SERIALIZED_SIZE: usize = 305;

        if data.len() < 8 {
            return Err(ErrorCode::InvalidAccount.into());
        }

        let maybe_discm: [u8; 8] = data[0..8].try_into().unwrap();

        if maybe_discm != GLOBAL_CONFIG_ACCOUNT_DISCM {
            return Err(ErrorCode::InvalidAccount.into());
        }

        let result: GlobalConfig = GlobalConfig::try_from_slice(&data[8..8 + SERIALIZED_SIZE])
            .map_err(|_| error!(ErrorCode::InvalidAccount))?;
        Ok(result)
    }

    pub fn deserialize_fees(data: &[u8]) -> Result<(u64, u64, u64)> {
        if data.len() < 8 {
            return Err(ErrorCode::InvalidAccount.into());
        }

        let maybe_discm: [u8; 8] = data[0..8].try_into().unwrap();
        if maybe_discm != GLOBAL_CONFIG_ACCOUNT_DISCM {
            return Err(ErrorCode::InvalidAccount.into());
        }

        // Оффсеты после дискриминатора:
        // admin: Pubkey = 32 bytes
        let mut offset = 8 + 32;

        // lp_fee_basis_points: u64 (8 bytes)
        let lp_fee_basis_points = u64::from_le_bytes(
            data[offset..offset + 8]
                .try_into()
                .map_err(|_| error!(ErrorCode::InvalidAccount))?,
        );

        offset = offset + 8;

        // protocol_fee_basis_points: u64 (8 bytes)
        let protocol_fee_basis_points = u64::from_le_bytes(
            data[offset..offset + 8]
                .try_into()
                .map_err(|_| error!(ErrorCode::InvalidAccount))?,
        );

        offset = offset + 8 + 1 + 8 * 32;

        // protocol_fee_basis_points: u64 (8 bytes)
        let coin_creator_fee_basis_points = u64::from_le_bytes(
            data[offset..offset + 8]
                .try_into()
                .map_err(|_| error!(ErrorCode::InvalidAccount))?,
        );

        Ok((
            lp_fee_basis_points,
            protocol_fee_basis_points,
            coin_creator_fee_basis_points,
        ))
    }
}

pub const PF_AMM_POOL_ACCOUNT_DISCM: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];
#[derive(Clone, Copy, BorshDeserialize)]
pub struct PfAmmPool {
    pub pool_bump: u8,
    pub index: u16,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub pool_base_token_account: Pubkey,
    pub pool_quote_token_account: Pubkey,
    /// True circulating supply without burns and lock-ups
    pub lp_supply: u64,
    pub coin_creator: Pubkey,
}

impl PfAmmPool {
    pub fn deserialize_data(data: &[u8]) -> Result<PfAmmPool> {
        pub const SERIALIZED_SIZE: usize = 203;

        if data.len() < 8 {
            return Err(ErrorCode::InvalidAccount.into());
        }

        let maybe_discm: [u8; 8] = data[0..8].try_into().unwrap();

        if maybe_discm != PF_AMM_POOL_ACCOUNT_DISCM {
            return Err(ErrorCode::InvalidAccount.into());
        }

        let result: PfAmmPool = PfAmmPool::try_from_slice(&data[8..8 + SERIALIZED_SIZE])
            .map_err(|_| error!(ErrorCode::InvalidAccount))?;

        Ok(result)
    }

    pub fn deserialize_mints(data: &[u8]) -> Result<(Pubkey, Pubkey)> {
        if data.len() < 8 {
            return Err(ErrorCode::InvalidAccount.into());
        }

        let maybe_discm: [u8; 8] = data[0..8].try_into().unwrap();
        if maybe_discm != PF_AMM_POOL_ACCOUNT_DISCM {
            return Err(ErrorCode::InvalidAccount.into());
        }

        let base_mint_offset = 8 + 3 + 32;
        let quote_mint_offset = base_mint_offset + 32;

        let base_mint = Pubkey::new_from_array(
            data[base_mint_offset..base_mint_offset + 32]
                .try_into()
                .map_err(|_| ErrorCode::InvalidAccount)?,
        );
        let quote_mint = Pubkey::new_from_array(
            data[quote_mint_offset..quote_mint_offset + 32]
                .try_into()
                .map_err(|_| ErrorCode::InvalidAccount)?,
        );

        Ok((base_mint, quote_mint))
    }
}

#[derive(Debug, Clone, Copy, BorshDeserialize)]
pub struct Fees {
    pub lp_fee_bps: u64,
    pub protocol_fee_bps: u64,
    pub creator_fee_bps: u64,
}

#[derive(Debug, Clone, Copy, BorshDeserialize)]
pub struct FeeTier {
    pub market_cap_lamports_threshold: u128,
    pub fees: Fees,
}

#[derive(Debug, Clone, BorshDeserialize)]
pub struct FeeConfig {
    pub bump: u8,
    pub admin: Pubkey,
    pub flat_fees: Fees,
    pub fee_tiers: Vec<FeeTier>,
}

impl FeeConfig {
    pub fn deserialize_data(data: &[u8]) -> Result<FeeConfig> {
        if data.len() < 8 {
            return Err(ErrorCode::InvalidAccount.into());
        }

        let mut cursor = &data[8..];

        let result: FeeConfig =
            FeeConfig::deserialize(&mut cursor).map_err(|_| ErrorCode::InvalidAccount)?;
        Ok(result)
    }
}
