use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::Token, token_2022::Token2022};

use crate::state::MEMO_PROGRAM_ID;

#[derive(Accounts)]
pub struct CommonAccounts<'info> {
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Program<'info, Token>,
    pub token_program_2022: Program<'info, Token2022>,

    /// CHECK:
    #[account(address = MEMO_PROGRAM_ID)]
    pub memo_program: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK:
    pub user_source_token_mint: AccountInfo<'info>,
    /// CHECK:
    pub user_intermediate_token_mint: AccountInfo<'info>,

    // #[account(
    //     init_if_needed,
    //     payer = user,
    //     associated_token::mint = user_source_token_mint,
    //     associated_token::authority = user,
    //     associated_token::token_program = token_program,
    // )]
    ///CHECK:
    #[account(mut)]
    pub user_source_token_account: AccountInfo<'info>,

    // #[account(
    //     init_if_needed,
    //     payer = user,
    //     associated_token::mint = user_intermediate_token_mint,
    //     associated_token::authority = user,
    //     associated_token::token_program = token_program,
    // )]
    ///CHECK:
    #[account(mut)]
    pub user_intermediate_token_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PumpswapAccounts<'info> {
    /// CHECK:
    pub program_id: AccountInfo<'info>,
    /// CHECK:
    pub pool: AccountInfo<'info>,
    /// CHECK:
    pub global_config: AccountInfo<'info>,
    /// CHECK:
    pub base_mint: AccountInfo<'info>,
    /// CHECK:
    pub quote_mint: AccountInfo<'info>,
    ///CHECK:
    #[account(mut)]
    pub base_token_account: AccountInfo<'info>,
    ///CHECK:
    #[account(mut)]
    pub quote_token_account: AccountInfo<'info>,
    /// CHECK:
    pub protocol_fee_recipient: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub protocol_fee_recipient_token_account: AccountInfo<'info>,
    /// CHECK:
    pub event_authority: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub creator_vault: AccountInfo<'info>,
    /// CHECK:
    pub creator: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub global_volume_accumulator: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub user_volume_accumulator: AccountInfo<'info>,
    /// CHECK:
    pub fee_config: AccountInfo<'info>,
    /// CHECK:
    pub fee_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RaydiumAmmAccounts<'info> {
    /// CHECK:
    pub program_id: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub amm_info: AccountInfo<'info>,
    /// CHECK:
    pub amm_authority: AccountInfo<'info>,
    ///CHECK:
    #[account(mut)]
    pub coin_vault: AccountInfo<'info>,
    ///CHECK:
    #[account(mut)]
    pub pc_vault: AccountInfo<'info>,
}
