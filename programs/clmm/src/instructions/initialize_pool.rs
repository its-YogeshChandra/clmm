use crate::states::pool::LpPoolStateShape;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self},
    token_interface::{self, Burn, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked},
};

//initilalize struct
#[derive(Accounts)]
pub struct Initialize<'info> {
    //signer of the pool
    #[account(mut)]
    pub signer: Signer<'info>,

    pub usdc_mint: InterfaceAccount<'info, Mint>,
    pub wsol_mint: InterfaceAccount<'info, Mint>,

    //token program
    pub token_program: Interface<'info, TokenInterface>,

    //system program
    pub system_program: Program<'info, System>,

    // make the pool state
    #[account(init, payer = signer, space= 8+LpPoolStateShape::INIT_SPACE, seeds = [b"pool_state_v1", usdc_mint.key().as_ref(), wsol_mint.key().as_ref()], bump)]
    pub pool_state_account: Account<'info, LpPoolStateShape>,

    //make the vaults
    #[account(init, payer=signer, token::mint = usdc_mint, token::authority = pool_state_account, token::token_program = token_program, seeds = [b"usdc_vault", usdc_mint.key().as_ref()], bump)]
    pub usdc_vault_account: InterfaceAccount<'info, TokenAccount>,

    #[account(init, payer=signer, token::mint = usdc_mint, token::authority = pool_state_account, token::token_program = token_program, seeds = [b"wsol_vault", wsol_mint.key().as_ref()], bump)]
    pub wsol_vault_account: InterfaceAccount<'info, TokenAccount>,

    // creating mint for the lp tokens
    #[account(init, payer = signer, mint::decimals= 9, mint::authority = pool_state_account, mint::freeze_authority = pool_state_account)]
    pub lp_token_mint: InterfaceAccount<'info, Mint>,
}
