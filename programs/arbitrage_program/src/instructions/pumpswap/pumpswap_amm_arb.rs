use std::u64;

use crate::arbitrage_engine::{
    find_optimal_amount, BasePool, ComputedParams, UniversalOptimalAmountParams,
};
use crate::commons::{create_ata_if_missing, get_ata_balance};
use crate::dex::pumpswap::pumpswap_pool::PumpswapPool;
use crate::dex::raydium_amm::raydium_amm_pool::RaydiumAmmPool;

use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct PumpswapAmmArb<'info> {
    pub common: CommonAccounts<'info>,
    pub pumpswap: PumpswapAccounts<'info>,
    pub amm: RaydiumAmmAccounts<'info>,
}

pub fn pumpswap_amm_arb<'b, 'info>(
    ctx: Context<'_, 'b, '_, 'info, PumpswapAmmArb<'info>>,
    params: UniversalOptimalAmountParams,
) -> Result<()> {
    // Создаем экземпляр PumpswapPool
    let pumpswap_pool = PumpswapPool::new(&ctx.accounts.common, &ctx.accounts.pumpswap)?;

    // Создаем экземпляр RaydiumAmmPool
    let amm_pool = RaydiumAmmPool::new(&ctx.accounts.common, &ctx.accounts.amm)?;

    let max_amount_in = get_ata_balance(&ctx.accounts.common.user_source_token_account)?;
    let max_amount_in = max_amount_in * 99 / 100;
    let computed_params = ComputedParams { max_amount_in };

    let result = find_optimal_amount(&pumpswap_pool, &amm_pool, &params, &computed_params)?;

    // Создаем ATA, если не существует
    create_ata_if_missing(
        &ctx.accounts.common.user,
        &ctx.accounts.common.user_intermediate_token_account,
        &ctx.accounts.common.user_intermediate_token_mint,
        &ctx.accounts.common.system_program,
        &ctx.accounts.common.token_program,
        &ctx.accounts.common.associated_token_program,
    )?;

    if result.direction_a_to_b {
        pumpswap_pool.swap(u64::MAX, result.intermediate_amount, true)?;
        amm_pool.swap(result.intermediate_amount, 0, false)?;
    } else {
        amm_pool.swap(result.amount_in, 0, true)?;
        pumpswap_pool.swap(result.intermediate_amount, 0, false)?;
    }

    Ok(())
}
