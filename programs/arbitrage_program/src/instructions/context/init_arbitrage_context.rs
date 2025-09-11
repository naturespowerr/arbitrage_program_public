use crate::commons::init_arbitrage_context;
use crate::state::*;
use anchor_lang::prelude::*;

pub fn init_arbitrage_context_instruction(
    ctx: Context<InitArbitrageContextAccounts>,
) -> Result<()> {
    init_arbitrage_context(ctx)
}
