use crate::commons::verify_arbitrage_context;
use crate::state::*;
use anchor_lang::prelude::*;

pub fn verify_arbitrage_context_instruction(ctx: Context<VerifyArbitrageAccounts>) -> Result<()> {
    verify_arbitrage_context(ctx)
}
