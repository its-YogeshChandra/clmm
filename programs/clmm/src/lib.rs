use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

// Keep modules for helper functions and logic
pub mod errors;
pub mod libraries;
pub mod states;

// Re-export states for use in accounts
use states::pool::LpPoolStateShape;
use states::position::Position;
use states::tick::TickArrayState;

declare_id!("CicZMzrBxTazWhSXGKXkkbnRiYFXSXm2Pe47RvW1X3qt");

#[program]
pub mod clmm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Initializing CLMM pool");
        // Store mints and vault addresses in pool state
        ctx.accounts.pool_state_account.token_0_mint = ctx.accounts.token_0_mint.key();
        ctx.accounts.pool_state_account.token_1_mint = ctx.accounts.token_1_mint.key();
        ctx.accounts.pool_state_account.token_0_vault_address = ctx.accounts.token_0_vault.key();
        ctx.accounts.pool_state_account.token_1_vault_address = ctx.accounts.token_1_vault.key();
        Ok(())
    }

    pub fn open_position(
        ctx: Context<OpenPosition>,
        tick_lower: i32,
        tick_upper: i32,
    ) -> Result<()> {
        let position = &mut ctx.accounts.position;
        position.pool_id = ctx.accounts.pool_state_account.key();
        position.owner = ctx.accounts.signer.key();
        position.tick_lower = tick_lower;
        position.tick_upper = tick_upper;
        position.liquidity = 0;
        position.tokens_owed_0 = 0;
        position.tokens_owed_1 = 0;
        position.fee_growth_inside_0_last = 0;
        position.fee_growth_inside_1_last = 0;
        Ok(())
    }
}

// Define all account structs in lib.rs for Anchor macro to find

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub token_0_mint: InterfaceAccount<'info, Mint>,
    pub token_1_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,

    #[account(
        init, 
        payer = signer, 
        space = 8 + LpPoolStateShape::INIT_SPACE, 
        seeds = [b"pool_state_v1", token_0_mint.key().as_ref(), token_1_mint.key().as_ref()], 
        bump
    )]
    pub pool_state_account: Account<'info, LpPoolStateShape>,

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

#[derive(Accounts)]
#[instruction(tick_lower: i32, tick_upper: i32)]
pub struct OpenPosition<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub pool_state_account: Account<'info, LpPoolStateShape>,

    #[account(
        init, 
        payer = signer, 
        space = 8 + Position::INIT_SPACE, 
        seeds = [b"position", signer.key().as_ref(), pool_state_account.key().as_ref(), &tick_upper.to_le_bytes(), &tick_lower.to_le_bytes()], 
        bump
    )]
    pub position: Account<'info, Position>,
}
