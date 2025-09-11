use crate::dex::pumpswap::interfaces::accounts::{FeeConfig, FeeTier, Fees};
use crate::error::ErrorCode;
use crate::Result;

pub fn pool_market_cap(
    base_mint_supply: u64,
    base_reserve: u64,
    quote_reserve: u64,
) -> Result<u128> {
    if base_reserve == 0 {
        return Err(ErrorCode::InvalidAccount.into());
    }

    let market_cap = (quote_reserve as u128)
        .checked_mul(base_mint_supply as u128)
        .ok_or(ErrorCode::InvalidAccount)?
        .checked_div(base_reserve as u128)
        .ok_or(ErrorCode::InvalidAccount)?;

    Ok(market_cap)
}

pub fn compute_fees_bps(
    fee_config: FeeConfig,
    base_mint_supply: u64,
    base_reserve: u64,
    quote_reserve: u64,
) -> Result<Fees> {
    let market_cap = pool_market_cap(base_mint_supply, base_reserve, quote_reserve)?;

    calculate_fee_tier(&fee_config.fee_tiers, market_cap)
}

fn calculate_fee_tier(fee_tiers: &[FeeTier], market_cap: u128) -> Result<Fees> {
    if fee_tiers.is_empty() {
        return Err(ErrorCode::InvalidAccount.into());
    }

    let first_tier = &fee_tiers[0];

    if market_cap < first_tier.market_cap_lamports_threshold {
        return Ok(first_tier.fees);
    }

    for tier in fee_tiers.iter().rev() {
        if market_cap >= tier.market_cap_lamports_threshold {
            return Ok(tier.fees);
        }
    }

    Ok(first_tier.fees)
}
