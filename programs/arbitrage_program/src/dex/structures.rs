#[derive(Debug, Clone, Copy)]
pub struct AmountOutResult {
    pub amount_out: u64,
    pub new_price: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct AmountInResult {
    pub amount_in: u64,
    pub new_price: f64,
}
