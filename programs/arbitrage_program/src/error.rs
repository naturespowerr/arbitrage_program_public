use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,

    #[msg("Unable to find bin array for pool")]
    MeteoraUnknownBinArray,

    #[msg("Unable to get active bin_array in provided accounts")]
    MeteoraNoBinArray,

    #[msg("Unable to get active bin in bin_Array")]
    MeteoraNoBinInArray,

    #[msg("No arbitrage opportunity")]
    NoArbitrageOpportunity,

    #[msg("Invalid account provided. Unable to deserialize")]
    InvalidAccount,

    #[msg("Token mint mismatch")]
    TokenMintMismatch,

    #[msg("Arithmetic overflow")]
    Overflow,

    #[msg("Arbitrage verification failed")]
    ArbitrageVerificationFailed,
    // #[msg("Division by zero")]
    // DivisionByZero,

    // #[msg("Missing Raydium accounts")]
    // MissingRaydiumAmmAccounts,
}
