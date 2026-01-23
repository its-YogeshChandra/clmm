use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self},
    token_interface::{self, Burn, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked},
};

use crate::states::{tick::TickArrayState, LpPoolStateShape};

#[derive(Accounts)]
pub struct Swap<'info> {
    //signer
    #[account(mut)]
    pub signer: Signer<'info>,

    //figure the issue
    #[account(mut)]
    pub pool_state_account: Account<'info, LpPoolStateShape>,

    //token program
    pub token_program: Interface<'info, TokenInterface>,

    //mint for the tokens
    pub input_mint: InterfaceAccount<'info, Mint>,
    pub output_mint: InterfaceAccount<'info, Mint>,

    //user accounts
    #[account(mut, token::mint = token_0_mint, token::authority = signer)]
    pub user_input_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, token::mint = token_1_mint, token::authority = signer)]
    pub user_output_token_account: InterfaceAccount<'info, TokenAccount>,

    //vaults account
    #[account(mut, token::mint = input_mint, token::authority = pool_state_account)]
    pub input_vault_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, token::mint = output_mint, token::authority = pool_state_account)]
    pub output_vault_account: InterfaceAccount<'info, TokenAccount>,

    //position
    //can pass the constrain this way
    #[account(mut,    constraint = position.owner == signer.key(),
    constraint = position.pool_id == pool_state_account.key() )]
    pub position: Account<'info, Position>,

    //tick array upper
    //used account loader
    #[account(mut)]
    pub tick_array_lower: AccountLoader<'info, TickArrayState>,
    #[account(mut)]
    pub tick_array_upper: AccountLoader<'info, TickArrayState>,
}

impl<'info> Swap<'info> {
    fn swap_function(
        mut self,
        amount_in: u64,
        minimum_amount_out: u64,
        sqrt_price_limit: u64,
        zero_for_one: bool,
    ) -> Result<()> {
        //read from the states
        let sqrt_price_current = self.pool_state_account.sqrt_price_x64;
        let current_tick = self.pool_state_account.tick_current;
        let liquidity = self.pool_state_account.liquidity;
        let amount_remaining: amount_in;
        let amount_out_total = 0;
        let fee_amount_total = 0;

        // the main swap logic
        while amount_remaining > 0 {
            todo!();
        }
        Ok(())
    }
}
