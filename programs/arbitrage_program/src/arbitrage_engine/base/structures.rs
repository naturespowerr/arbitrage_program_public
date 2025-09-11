use anchor_lang::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LiquidityType {
    Concentrated,
    Constant,
}
#[derive(Debug, Copy, Clone)]
pub struct SwapResult {
    pub amount_out: u64,
    pub consumed_in_amount: u64,
    pub is_fully_filled: bool,
    pub new_price: f64,
}

/// Структура для возврата результата поиска оптимальной суммы
#[derive(Debug)]
pub struct OptimalAmountResult {
    pub amount_in: u64,
    pub intermediate_amount: u64,
    pub direction_a_to_b: bool,
    pub profit: i64,
}

/// Структура для передачи вычисленных параметров в функции подбора цены
#[derive(Debug, Clone, Copy)]
pub struct ComputedParams {
    pub max_amount_in: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct BaseSwapParams {
    pub amount_in: u64,
    pub min_amount_out: u64,
    pub source_to_intermediate: bool,
}
