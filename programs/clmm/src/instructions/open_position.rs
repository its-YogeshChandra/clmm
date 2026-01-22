use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self},
    token_interface::{self, Burn, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked},
};

use crate::states::pool::LpPoolStateShape;
use crate::states::Position;

//files
#[derive(Accounts)]
pub struct Open_Position<'info> {
    //signer for the value
    #[account(mut)]
    pub signer: Signer<'info>,

    //system program
    pub system_program: Program<'info, System>,

    //pool static
    pub pool_state_account: Account<'info, LpPoolStateShape>,

    //postion
    #[account(init, payer= signer, space= 8+Position::INIT_SPACE, seeds = [b"position", signer.key().as_ref()], bump)]
    pub position: Account<'info, Position>,
}
