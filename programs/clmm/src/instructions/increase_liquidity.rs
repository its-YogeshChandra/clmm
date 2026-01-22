use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self},
    token_interface::{self, Burn, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{states::Position, LpPoolStateShape, TickArrayState};

#[derive(Accounts)]
pub struct increase_liquidity<'info> {
    //signer
    #[account(mut)]
    pub signer: Signer<'info>,

    //figure the issue
    pub pool_state_account: Account<'info, LpPoolStateShape>,

    //mint for the tokens
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    pub wsol_mint: InterfaceAccount<'info, Mint>,

    //user accounts
    #[account(mut, token::mint = usdc_mint, token::authority = signer)]
    pub user_usdc_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, token::mint = wsol_mint, token::authority = signer)]
    pub user_wsol_account: InterfaceAccount<'info, TokenAccount>,

    //vaults account
    #[account(mut, token::mint = usdc_mint, token::authority = pool_state_account)]
    pub usdc_vault_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, token::mint = wsol_mint, token::authority = pool_state_account)]
    pub wsol_vault_account: InterfaceAccount<'info, TokenAccount>,

    //position
    pub position: Account<'info, Position>,

    //tick
    pub array: Account<'info, TickArrayState>,
}
