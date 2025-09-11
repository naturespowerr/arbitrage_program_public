use crate::commons::get_ata_balance;
use crate::state::arb_context_accounts::InitArbitrageContextAccounts;
use anchor_lang::prelude::*;

pub fn init_arbitrage_context(ctx: Context<InitArbitrageContextAccounts>) -> Result<()> {
    let arb_ctx = &mut ctx.accounts.arb_ctx;
    let user = &ctx.accounts.user;
    let user_source_token_account = &ctx.accounts.user_source_token_account;

    arb_ctx.bump = ctx.bumps.arb_ctx;
    arb_ctx.owner = user.key();

    arb_ctx.start_sol = user.lamports();
    arb_ctx.start_src = get_ata_balance(user_source_token_account)?;

    Ok(())
}
