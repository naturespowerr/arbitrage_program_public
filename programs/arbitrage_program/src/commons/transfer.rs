use anchor_lang::prelude::*;

pub fn invoke_transfer<'b, 'info>(
    from: &'b Signer<'info>,
    to: &'b AccountInfo<'info>,
    system_program: &'b Program<'info, System>,
    amount: u64,
) -> Result<()> {
    // Создаем инструкцию для трансфера
    let ix = anchor_lang::solana_program::system_instruction::transfer(from.key, to.key, amount);

    // Выполняем инструкцию
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            from.to_account_info(),
            to.to_account_info(),
            system_program.to_account_info(),
        ],
    )?;

    Ok(())
}
