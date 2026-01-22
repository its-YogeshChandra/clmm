use anchor_lang::prelude::*;

#[error_code]
pub enum TickMathError {
    #[msg("tick is higher then eligible amount")]
    TickUpperOverflow,
}
