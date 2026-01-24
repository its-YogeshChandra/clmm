use crate::states::pool::LpPoolStateShape;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

//initialize struct
#[derive(Accounts)]
pub struct Initialize<'info> {
    //signer of the pool
    #[account(mut)]
    pub signer: Signer<'info>,

    /// Token 0 mint (e.g., USDC)
    pub token_0_mint: InterfaceAccount<'info, Mint>,
    /// Token 1 mint (e.g., WSOL)  
    pub token_1_mint: InterfaceAccount<'info, Mint>,

    //token program
    pub token_program: Interface<'info, TokenInterface>,

    //system program
    pub system_program: Program<'info, System>,

    // make the pool state
    #[account(
        init, 
        payer = signer, 
        space = 8 + LpPoolStateShape::INIT_SPACE, 
        seeds = [b"pool_state_v1", token_0_mint.key().as_ref(), token_1_mint.key().as_ref()], 
        bump
    )]
    pub pool_state_account: Account<'info, LpPoolStateShape>,

    //make the vaults
    #[account(
        init, 
        payer = signer, 
        token::mint = token_0_mint, 
        token::authority = pool_state_account, 
        token::token_program = token_program, 
        seeds = [b"token_0_vault", token_0_mint.key().as_ref()], 
        bump
    )]
    pub token_0_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init, 
        payer = signer, 
        token::mint = token_1_mint, 
        token::authority = pool_state_account, 
        token::token_program = token_program, 
        seeds = [b"token_1_vault", token_1_mint.key().as_ref()], 
        bump
    )]
    pub token_1_vault: InterfaceAccount<'info, TokenAccount>,
}

