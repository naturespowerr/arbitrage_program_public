use crate::arbitrage_engine::{BasePool, BaseSwapParams};
use crate::commons::create_ata_if_missing;
use crate::dex::raydium_amm::raydium_amm_pool::RaydiumAmmPool;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RaydiumAmmSwap<'info> {
    pub common: CommonAccounts<'info>,
    pub raydium_amm: RaydiumAmmAccounts<'info>,
}

pub fn raydium_amm_swap<'b, 'info>(
    ctx: Context<'_, 'b, '_, 'info, RaydiumAmmSwap<'info>>,
    params: BaseSwapParams,
) -> Result<()> {
    let raydium_amm_pool = RaydiumAmmPool::new(&ctx.accounts.common, &ctx.accounts.raydium_amm)?;

    let test_result =
        raydium_amm_pool.get_amount_out(params.amount_in, !params.source_to_intermediate)?;

    msg!(
        "get_amount_out: amount_in={}, amount_out={}, new_price={}, swap_for_quote={}",
        params.amount_in,
        test_result.amount_out,
        test_result.new_price,
        !params.source_to_intermediate
    );

    msg!(
        "current_price={}, fee_rate={}",
        raydium_amm_pool.get_price()?,
        raydium_amm_pool.get_fee_rate_f64()
    );

    create_ata_if_missing(
        &ctx.accounts.common.user,
        &ctx.accounts.common.user_intermediate_token_account,
        &ctx.accounts.common.user_intermediate_token_mint,
        &ctx.accounts.common.system_program,
        &ctx.accounts.common.token_program,
        &ctx.accounts.common.associated_token_program,
    )?;

    raydium_amm_pool.swap(
        params.amount_in,
        params.min_amount_out,
        params.source_to_intermediate,
    )?;

    Ok(())
}
