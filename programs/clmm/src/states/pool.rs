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
    pub bump: u8,

    //configuration
    pub tick_spacing: u16,
    pub fee_rate: u32,

    //current state
    pub sqrt_price_x64: u128,
    pub tick_current: i32,
    pub liquidity: u128,

    //fee tracking
    pub fee_growth_global_0: u128,
    pub fee_growth_global_1: u128,
}
