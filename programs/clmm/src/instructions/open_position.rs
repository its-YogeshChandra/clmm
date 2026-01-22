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
#[instruction(tick_lower: i32, tick_upper: i32)]
pub struct Open_Position<'info> {
    //signer for the value
    #[account(mut)]
    pub signer: Signer<'info>,

    //system program
    pub system_program: Program<'info, System>,

    //pool static
    pub pool_state_account: Account<'info, LpPoolStateShape>,

    //postion
    #[account(init, payer= signer, space= 8+Position::INIT_SPACE, seeds = [b"position", signer.key().as_ref(), pool_state_account.key().as_ref(),&tick_upper.to_le_bytes(), &tick_lower.to_le_bytes()], bump)]
    pub position: Account<'info, Position>,
}

impl<'info> Open_Position<'info> {
    pub fn handler(&mut self, tick_lower: i32, tick_upper: i32) -> Result<()> {
        let position = &mut self.position;
        position.pool_id = self.pool_state_account.key();
        position.owner = self.signer.key();
        position.tick_lower = tick_lower;
        position.liquidity = 0;
        position.tick_upper = tick_upper;
        position.tokens_owed_0 = 0;
        position.tokens_owed_1 = 0;
        position.fee_growth_inside_0_last = 0;
        position.fee_growth_inside_1_last = 0;
        Ok(())
    }
}
