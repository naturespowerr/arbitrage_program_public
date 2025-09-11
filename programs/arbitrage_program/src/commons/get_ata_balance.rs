use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

pub fn get_ata_balance(ata_account: &AccountInfo) -> Result<u64> {
    let token_account = TokenAccount::try_deserialize(&mut ata_account.data.borrow().as_ref())?;
    Ok(token_account.amount)
}
