use anchor_lang::prelude::*;

use super::{LiquidityType, SwapResult};

pub trait BasePool {
    fn get_fee_rate_f64(&self) -> f64;
    fn get_price(&self) -> Result<f64>;
    fn get_amount_out(&self, amount_in: u64, swap_for_quote: bool) -> Result<SwapResult>;
    fn get_amount_in(&self, amount_out: u64, swap_for_quote: bool) -> Result<SwapResult>;
    fn get_pool_type(&self) -> Result<LiquidityType>;

    fn get_max_amount(&self, price_delta: f64, swap_for_quote: bool) -> Result<u64>;

    fn get_base_quote_product(&self) -> Result<u128>;

    fn swap(&self, amount_in: u64, min_amount_out: u64, source_to_intermediate: bool)
        -> Result<()>;
}
