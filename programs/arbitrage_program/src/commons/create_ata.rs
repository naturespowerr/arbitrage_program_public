use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::associated_token::{create, Create};
use anchor_spl::token::Token;

pub fn create_ata_if_missing<'info>(
    payer: &Signer<'info>,
    ata_account: &AccountInfo<'info>,
    mint: &AccountInfo<'info>,
    system_program: &Program<'info, System>,
    token_program: &Program<'info, anchor_spl::token::Token>,
    associated_token_program: &Program<'info, AssociatedToken>,
) -> Result<bool> {
    if !ata_account.data_is_empty() {
        return Ok(false);
    }

    let cpi_accounts = Create {
        payer: payer.to_account_info(),
        associated_token: ata_account.clone(),
        authority: payer.to_account_info(),
        mint: mint.clone(),
        system_program: system_program.to_account_info(),
        token_program: token_program.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(associated_token_program.to_account_info(), cpi_accounts);

    create(cpi_ctx)?;

    Ok(true)
}

pub fn create_atas_if_missing<'info>(
    payer: &Signer<'info>,
    ata_mint_pairs: Vec<(&AccountInfo<'info>, &AccountInfo<'info>)>,
    system_program: &Program<'info, System>,
    token_program: &Program<'info, Token>,
    associated_token_program: &Program<'info, AssociatedToken>,
) -> Result<Vec<bool>> {
    let mut created_flags = Vec::new();

    for (ata_account, mint) in ata_mint_pairs {
        if ata_account.data_is_empty() {
            let cpi_accounts = Create {
                payer: payer.to_account_info(),
                associated_token: ata_account.clone(),
                authority: payer.to_account_info(),
                mint: mint.clone(),
                system_program: system_program.to_account_info(),
                token_program: token_program.to_account_info(),
            };

            let cpi_ctx = CpiContext::new(associated_token_program.to_account_info(), cpi_accounts);
            create(cpi_ctx)?;
            created_flags.push(true);
        } else {
            created_flags.push(false);
        }
    }

    Ok(created_flags)
}
