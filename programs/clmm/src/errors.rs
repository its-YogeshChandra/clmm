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
}
