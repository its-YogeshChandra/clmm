use anchor_lang::prelude::*;

//design postions
#[account]
#[derive(InitSpace)]
pub struct Position {
    pool_id: Pubkey,
    owner: Pubkey,
    tick_lower: i32,
    tick_upper: i32,
    liquidity: u128,
    fee_growth_inside_0_last: u128,
    fee_growth_inside_1_last: u128,
    tokens_owed_0: u64,
    tokens_owed_1: u64,
}
