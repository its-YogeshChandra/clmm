use anchor_lang::prelude::*;

//design postions
#[account]
#[derive(InitSpace)]
pub struct Position {
    pub pool_id: Pubkey,
    pub owner: Pubkey,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub liquidity: u128,
    pub fee_growth_inside_0_last: u128,
    pub fee_growth_inside_1_last: u128,
    pub tokens_owed_0: u64,
    pub tokens_owed_1: u64,
}
