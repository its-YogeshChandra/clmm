use anchor_lang::prelude::*;

//include ticks
#[account]
#[derive(InitSpace)]
pub struct LpPoolStateShape {
    //mint of both vauts
    pub usdc_mint: Pubkey,
    pub wsol_mint: Pubkey,
    //vault addresses
    pub usdc_vault_address: Pubkey,
    pub sol_vault_address: Pubkey,
    //mint
    pub lpmint: Pubkey,
    //bump
    bump: u8,

    //configuration
    tick_spacing: u16,
    fee_rate: u32,

    //current state
    sqrt_price_x64: u128,
    tick_current: i32,
    liquidity: u128,

    //fee tracking
    fee_growth_global_0: u128,
    fee_growth_global_1: u128,
}
