use anchor_lang::prelude::*;

#[error_code]
pub enum TickMathError {
    #[msg("tick is higher then eligible amount")]
    TickUpperOverflow,
}

#[error_code]
pub enum LiquidityError {
    #[msg("zero Liquidity Error")]
    ZeroLiquidity,

    #[msg("insufficient liquidity")]
    InsufficientLiquidity,
}

#[error_code]
pub enum SwapError {
    #[msg("invalid square root price limit")]
    InvalidSqrtPriceLimit,

    #[msg("no liquidity available in pool")]
    NoLiquidity,

    #[msg("swap amount must be greater than zero")]
    ZeroAmount,

    #[msg("output amount less than minimum")]
    TooLittleOutputReceived,
}

