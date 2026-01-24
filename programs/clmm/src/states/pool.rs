use anchor_lang::prelude::*;

//include ticks
#[account]
#[derive(InitSpace)]
pub struct LpPoolStateShape {
    //mint of both vaults
    pub token_0_mint: Pubkey,
    pub token_1_mint: Pubkey,
    //vault addresses
    pub token_0_vault_address: Pubkey,
    pub token_1_vault_address: Pubkey,
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

