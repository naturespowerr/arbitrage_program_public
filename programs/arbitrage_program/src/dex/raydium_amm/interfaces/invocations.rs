use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::invoke;

/// Executes a Raydium AMM V4 swap
pub fn raydium_amm_v4_swap<'info>(
    token_program: &AccountInfo<'info>,
    amm_program: &AccountInfo<'info>,
    amm_pool: &AccountInfo<'info>,
    amm_authority: &AccountInfo<'info>,
    amm_open_orders: &AccountInfo<'info>,
    amm_coin_vault: &AccountInfo<'info>,
    amm_pc_vault: &AccountInfo<'info>,
    market_program: &AccountInfo<'info>,
    market: &AccountInfo<'info>,
    market_bids: &AccountInfo<'info>,
    market_asks: &AccountInfo<'info>,
    market_event_queue: &AccountInfo<'info>,
    market_coin_vault: &AccountInfo<'info>,
    market_pc_vault: &AccountInfo<'info>,
    market_vault_signer: &AccountInfo<'info>,
    user_source: &AccountInfo<'info>,
    user_destination: &AccountInfo<'info>,
    user_wallet: &Signer<'info>,
    amount_in: u64,
    minimum_amount_out: u64,
) -> Result<()> {
    // Create the instruction data for Raydium's RaydiumAmmV4swapBaseIn
    let mut data = Vec::with_capacity(17);

    // Instruction discriminator for RaydiumAmmV4SwapBaseIn (9)
    data.push(9);

    // amount_in (u64)
    data.extend_from_slice(&amount_in.to_le_bytes());

    // minimum_amount_out (u64)
    data.extend_from_slice(&minimum_amount_out.to_le_bytes());

    // Create the instruction
    let ix = Instruction {
        program_id: amm_program.clone().key(),
        accounts: vec![
            AccountMeta::new_readonly(token_program.key(), false),
            AccountMeta::new(amm_pool.key(), false),
            AccountMeta::new_readonly(amm_authority.key(), false),
            AccountMeta::new(amm_open_orders.key(), false),
            AccountMeta::new(amm_coin_vault.key(), false),
            AccountMeta::new(amm_pc_vault.key(), false),
            AccountMeta::new_readonly(market_program.key(), false),
            AccountMeta::new(market.key(), false),
            AccountMeta::new(market_bids.key(), false),
            AccountMeta::new(market_asks.key(), false),
            AccountMeta::new(market_event_queue.key(), false),
            AccountMeta::new(market_coin_vault.key(), false),
            AccountMeta::new(market_pc_vault.key(), false),
            AccountMeta::new_readonly(market_vault_signer.key(), false),
            AccountMeta::new(user_source.key(), false),
            AccountMeta::new(user_destination.key(), false),
            AccountMeta::new_readonly(user_wallet.key(), true),
        ],
        data,
    };

    // Get all the account infos
    let account_infos = &[
        token_program.to_account_info().clone(),
        amm_pool.clone(),
        amm_authority.clone(),
        amm_open_orders.clone(),
        amm_coin_vault.to_account_info().clone(),
        amm_pc_vault.to_account_info().clone(),
        market_program.clone(),
        market.clone(),
        market_bids.clone(),
        market_asks.clone(),
        market_event_queue.clone(),
        market_coin_vault.to_account_info().clone(),
        market_pc_vault.to_account_info().clone(),
        market_vault_signer.clone(),
        user_source.to_account_info().clone(),
        user_destination.to_account_info().clone(),
        user_wallet.to_account_info(),
    ];

    // Execute the instruction
    invoke(&ix, account_infos)?;

    Ok(())
}
