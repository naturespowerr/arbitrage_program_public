use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ArbCtx {
    pub bump: u8,
    pub owner: Pubkey,

    // стартовые балансы
    pub start_sol: u64,
    pub start_src: u64,
}

#[derive(Accounts)]
pub struct InitArbitrageContextAccounts<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub user: Signer<'info>,

    ///CHECK:
    #[account(mut)]
    pub user_source_token_account: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = user,
        seeds = [b"context", user.key().as_ref()],
        bump,
        space = 8 + ArbCtx::INIT_SPACE,
    )]
    pub arb_ctx: Account<'info, ArbCtx>,
}

// объявляем константу
pub const MEMO_PROGRAM_ID: Pubkey = pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");

#[derive(Accounts)]
pub struct VerifyArbitrageAccounts<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub user: Signer<'info>,

    ///CHECK:
    #[account(mut)]
    pub user_source_token_account: AccountInfo<'info>,

    // #[account(
    //     mut,
    //     close = user, // вернём ренту пользователю
    //     seeds = [b"context", user.key().as_ref()],
    //     bump = arb_ctx.bump,
    //     constraint = arb_ctx.owner == user.key(),
    // )]
    pub arb_ctx: Account<'info, ArbCtx>,
}
