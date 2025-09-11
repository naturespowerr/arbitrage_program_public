use crate::commons::get_ata_balance;
use crate::error::ErrorCode;
use crate::state::arb_context_accounts::VerifyArbitrageAccounts;
use anchor_lang::prelude::*;

pub fn verify_arbitrage_context(ctx: Context<VerifyArbitrageAccounts>) -> Result<()> {
    let arb_ctx = &ctx.accounts.arb_ctx;

    let user = &ctx.accounts.user;

    let curr_sol = user.lamports();
    let curr_src = get_ata_balance(&ctx.accounts.user_source_token_account)?;

    let start_total = arb_ctx.start_sol + arb_ctx.start_src;
    let curr_total = curr_sol + curr_src;

    require!(
        curr_total >= start_total,
        ErrorCode::ArbitrageVerificationFailed
    );

    Ok(())
}
