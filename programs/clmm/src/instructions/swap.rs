use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self},
    token_interface::{self, Burn, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked},
};

use crate::states::tick::TickArrayState;

#[derive(Accounts)]
pub struct Swap<'info> {
    //account
    #[account(mut)]
    pub signer: Signer<'info>,

    //pool state
    pub pool_state_account: InterfaceAccount<'info, TokenAccount>,

    //user accounts
    #[account(mut, token::authority = signer)]
    pub user_input_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, token::authority = signer)]
    pub user_output_account: InterfaceAccount<'info, TokenAccount>,

    //vault account
    #[account(mut, token::authority = pool_state_account)]
    pub input_vault_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut , token::authority = pool_state_account)]
    pub output_vault_account: InterfaceAccount<'info, TokenAccount>,

    //token account
    pub tickarray: Account<'info, TickArrayState>,
}
