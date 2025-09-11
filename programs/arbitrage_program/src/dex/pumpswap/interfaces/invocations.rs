use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::invoke;

/// Executes a PumpSwap buy operation
/// This function buys base tokens using quote tokens
pub fn pumpswap_buy<'info>(
    program: &AccountInfo<'info>,
    pool: &AccountInfo<'info>,
    user: &Signer<'info>,
    global_config: &AccountInfo<'info>,
    base_mint: &AccountInfo<'info>,
    quote_mint: &AccountInfo<'info>,
    user_base_token_account: &AccountInfo<'info>,
    user_quote_token_account: &AccountInfo<'info>,
    pool_base_token_account: &AccountInfo<'info>,
    pool_quote_token_account: &AccountInfo<'info>,
    protocol_fee_recipient: &AccountInfo<'info>,
    protocol_fee_recipient_token_account: &AccountInfo<'info>,
    base_token_program: &AccountInfo<'info>,
    quote_token_program: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    associated_token_program: &AccountInfo<'info>,
    event_authority: &AccountInfo<'info>,
    creator_vault: &AccountInfo<'info>,
    creator: &AccountInfo<'info>,
    global_volume_accumulator: &AccountInfo<'info>,
    user_volume_accumulator: &AccountInfo<'info>,
    fee_config: &AccountInfo<'info>,
    fee_program: &AccountInfo<'info>,
    base_amount_out: u64,
    max_quote_amount_in: u64,
) -> Result<()> {
    // Создаем данные для инструкции buy
    let mut data = Vec::with_capacity(24); // 8 байт дискриминатор + 16 байт для двух u64 аргументов

    // Дискриминатор инструкции buy из IDL
    data.extend_from_slice(&[102, 6, 61, 18, 1, 218, 235, 234]);

    // base_amount_out (u64)
    data.extend_from_slice(&base_amount_out.to_le_bytes());

    // max_quote_amount_in (u64)
    data.extend_from_slice(&max_quote_amount_in.to_le_bytes());

    // Создаем инструкцию
    let ix = Instruction {
        program_id: program.key(),
        accounts: vec![
            AccountMeta::new_readonly(pool.key(), false),
            AccountMeta::new(user.key(), true),
            AccountMeta::new_readonly(global_config.key(), false),
            AccountMeta::new_readonly(base_mint.key(), false),
            AccountMeta::new_readonly(quote_mint.key(), false),
            AccountMeta::new(user_base_token_account.key(), false),
            AccountMeta::new(user_quote_token_account.key(), false),
            AccountMeta::new(pool_base_token_account.key(), false),
            AccountMeta::new(pool_quote_token_account.key(), false),
            AccountMeta::new_readonly(protocol_fee_recipient.key(), false),
            AccountMeta::new(protocol_fee_recipient_token_account.key(), false),
            AccountMeta::new_readonly(base_token_program.key(), false),
            AccountMeta::new_readonly(quote_token_program.key(), false),
            AccountMeta::new_readonly(system_program.key(), false),
            AccountMeta::new_readonly(associated_token_program.key(), false),
            AccountMeta::new_readonly(event_authority.key(), false),
            AccountMeta::new_readonly(program.key(), false),
            AccountMeta::new(creator_vault.key(), false),
            AccountMeta::new_readonly(creator.key(), false),
            AccountMeta::new(global_volume_accumulator.key(), false),
            AccountMeta::new(user_volume_accumulator.key(), false),
            AccountMeta::new_readonly(fee_config.key(), false),
            AccountMeta::new_readonly(fee_program.key(), false),
        ],
        data,
    };

    // Получаем все account infos
    let account_infos = &[
        program.clone(),
        pool.clone(),
        user.to_account_info().clone(),
        global_config.clone(),
        base_mint.clone(),
        quote_mint.clone(),
        user_base_token_account.clone(),
        user_quote_token_account.clone(),
        pool_base_token_account.clone(),
        pool_quote_token_account.clone(),
        protocol_fee_recipient.clone(),
        protocol_fee_recipient_token_account.clone(),
        base_token_program.clone(),
        quote_token_program.clone(),
        system_program.clone(),
        associated_token_program.clone(),
        event_authority.clone(),
        program.clone(),
        creator_vault.clone(),
        creator.clone(),
        global_volume_accumulator.clone(),
        user_volume_accumulator.clone(),
        fee_config.clone(),
        fee_program.clone(),
    ];

    // Выполняем инструкцию
    invoke(&ix, account_infos)?;

    Ok(())
}

/// Executes a PumpSwap sell operation
/// This function sells base tokens for quote tokens
pub fn pumpswap_sell<'info>(
    program: &AccountInfo<'info>,
    pool: &AccountInfo<'info>,
    user: &Signer<'info>,
    global_config: &AccountInfo<'info>,
    base_mint: &AccountInfo<'info>,
    quote_mint: &AccountInfo<'info>,
    user_base_token_account: &AccountInfo<'info>,
    user_quote_token_account: &AccountInfo<'info>,
    pool_base_token_account: &AccountInfo<'info>,
    pool_quote_token_account: &AccountInfo<'info>,
    protocol_fee_recipient: &AccountInfo<'info>,
    protocol_fee_recipient_token_account: &AccountInfo<'info>,
    base_token_program: &AccountInfo<'info>,
    quote_token_program: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    associated_token_program: &AccountInfo<'info>,
    event_authority: &AccountInfo<'info>,
    creator_vault: &AccountInfo<'info>,
    creator: &AccountInfo<'info>,
    fee_config: &AccountInfo<'info>,
    fee_program: &AccountInfo<'info>,
    base_amount_in: u64,
    min_quote_amount_out: u64,
) -> Result<()> {
    // Создаем данные для инструкции sell
    let mut data = Vec::with_capacity(24); // 8 байт дискриминатор + 16 байт для двух u64 аргументов

    // Дискриминатор инструкции sell из IDL
    data.extend_from_slice(&[51, 230, 133, 164, 1, 127, 131, 173]);

    // base_amount_in (u64)
    data.extend_from_slice(&base_amount_in.to_le_bytes());

    // min_quote_amount_out (u64)
    data.extend_from_slice(&min_quote_amount_out.to_le_bytes());

    // Создаем инструкцию
    let ix = Instruction {
        program_id: program.key(),
        accounts: vec![
            AccountMeta::new_readonly(pool.key(), false),
            AccountMeta::new(user.key(), true),
            AccountMeta::new_readonly(global_config.key(), false),
            AccountMeta::new_readonly(base_mint.key(), false),
            AccountMeta::new_readonly(quote_mint.key(), false),
            AccountMeta::new(user_base_token_account.key(), false),
            AccountMeta::new(user_quote_token_account.key(), false),
            AccountMeta::new(pool_base_token_account.key(), false),
            AccountMeta::new(pool_quote_token_account.key(), false),
            AccountMeta::new_readonly(protocol_fee_recipient.key(), false),
            AccountMeta::new(protocol_fee_recipient_token_account.key(), false),
            AccountMeta::new_readonly(base_token_program.key(), false),
            AccountMeta::new_readonly(quote_token_program.key(), false),
            AccountMeta::new_readonly(system_program.key(), false),
            AccountMeta::new_readonly(associated_token_program.key(), false),
            AccountMeta::new_readonly(event_authority.key(), false),
            AccountMeta::new_readonly(program.key(), false),
            AccountMeta::new(creator_vault.key(), false),
            AccountMeta::new_readonly(creator.key(), false),
            AccountMeta::new_readonly(fee_config.key(), false),
            AccountMeta::new_readonly(fee_program.key(), false),
        ],
        data,
    };

    // Получаем все account infos
    let account_infos = &[
        program.clone(),
        pool.clone(),
        user.to_account_info().clone(),
        global_config.clone(),
        base_mint.clone(),
        quote_mint.clone(),
        user_base_token_account.clone(),
        user_quote_token_account.clone(),
        pool_base_token_account.clone(),
        pool_quote_token_account.clone(),
        protocol_fee_recipient.clone(),
        protocol_fee_recipient_token_account.clone(),
        base_token_program.clone(),
        quote_token_program.clone(),
        system_program.clone(),
        associated_token_program.clone(),
        event_authority.clone(),
        program.clone(),
        creator_vault.clone(),
        creator.clone(),
        fee_config.clone(),
        fee_program.clone(),
    ];

    // Выполняем инструкцию
    invoke(&ix, account_infos)?;

    Ok(())
}
