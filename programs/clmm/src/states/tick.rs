use anchor_lang::prelude::*;

//for managing the ticks
#[account]
pub struct TickState {
    initialized: bool,
    liquidity_gross: u128, //total liquidity referencing this
    liquidity_net: i128,
    fee_growth_outside_0: u128,
    fee_growth_outside_1: u128,
}

//tick state array
#[account]
pub struct TickArrayState {
    pool_id: Pubkey,
    start_tick_index: i32,
    ticks: [TickState; 60],
}
