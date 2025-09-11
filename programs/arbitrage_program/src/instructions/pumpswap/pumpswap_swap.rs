use crate::arbitrage_engine::{BasePool, BaseSwapParams};
use crate::commons::create_ata_if_missing;
use crate::dex::pumpswap::pumpswap_pool::PumpswapPool;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct PumpswapSwap<'info> {
    pub common: CommonAccounts<'info>,
    pub pumpswap: PumpswapAccounts<'info>,
}

pub fn pumpswap_swap<'b, 'info>(
    ctx: Context<'_, 'b, '_, 'info, PumpswapSwap<'info>>,
    params: BaseSwapParams,
) -> Result<()> {
    let pumpswap_pool = PumpswapPool::new(&ctx.accounts.common, &ctx.accounts.pumpswap)?;

    let test_result =
        pumpswap_pool.get_amount_out(params.amount_in, !params.source_to_intermediate)?;

    msg!(
        "get_amount_out: amount_in={}, amount_out={}, new_price={}, swap_for_quote={}",
        params.amount_in,
        test_result.amount_out,
        test_result.new_price,
        !params.source_to_intermediate
    );

    msg!(
        "current_price={}, fee_rate={}",
        pumpswap_pool.get_price()?,
        pumpswap_pool.get_fee_rate_f64()
    );

    create_ata_if_missing(
        &ctx.accounts.common.user,
        &ctx.accounts.common.user_intermediate_token_account,
        &ctx.accounts.common.user_intermediate_token_mint,
        &ctx.accounts.common.system_program,
        &ctx.accounts.common.token_program,
        &ctx.accounts.common.associated_token_program,
    )?;

    pumpswap_pool.swap(
        test_result.consumed_in_amount + 1,
        test_result.amount_out,
        params.source_to_intermediate,
    )?;

    Ok(())
}
