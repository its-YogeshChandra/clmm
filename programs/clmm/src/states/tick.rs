use anchor_lang::prelude::*;

//for managing the ticks
#[zero_copy(unsafe)]
#[repr(C, packed)]
#[derive(Default, Debug)]
pub struct TickState {
    pub initialized: u8,
    pub _padding: [u8; 7],
    pub liquidity_gross: u128, //total liquidity referencing this
    pub liquidity_net: i128,
    pub fee_growth_outside_0: u128,
    pub fee_growth_outside_1: u128,
}

//tick state array
#[account(zero_copy(unsafe))]
#[repr(C, packed)]
pub struct TickArrayState {
    pub pool_id: Pubkey,
    pub start_tick_index: i32,
    pub _padding: [u8; 4],
    pub ticks: [TickState; 60],
}
