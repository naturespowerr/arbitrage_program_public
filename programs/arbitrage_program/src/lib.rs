#![allow(unexpected_cfgs)]
#![allow(deprecated)]

pub mod arbitrage_engine;
pub mod commons;
pub mod dex;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use arbitrage_engine::{BaseSwapParams, UniversalOptimalAmountParams};

// Note: Using glob import here is necessary for the #[program] macro to work correctly
// even though it causes an "ambiguous glob re-exports" warning

declare_id!("B1111111111111111111111111111111111111111111");

#[program]
pub mod arbitrage_program {
    use super::*;
    pub use instructions::*;

    pub fn raydium_amm_swap<'b, 'info>(
        ctx: Context<'_, 'b, '_, 'info, RaydiumAmmSwap<'info>>,
        params: BaseSwapParams,
    ) -> Result<()> {
        instructions::raydium_amm_swap(ctx, params)
    }
    pub fn pumpswap_swap<'b, 'info>(
        ctx: Context<'_, 'b, '_, 'info, PumpswapSwap<'info>>,
        params: BaseSwapParams,
    ) -> Result<()> {
        instructions::pumpswap_swap(ctx, params)
    }

    //-------------------------------------------------------------------

    pub fn pumpswap_amm_arb<'b, 'info>(
        ctx: Context<'_, 'b, '_, 'info, PumpswapAmmArb<'info>>,
        params: UniversalOptimalAmountParams,
    ) -> Result<()> {
        instructions::pumpswap_amm_arb(ctx, params)
    }
}
