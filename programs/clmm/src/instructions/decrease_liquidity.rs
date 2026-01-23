use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self},
    token_interface::{self, Burn, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked},
};

use crate::errors::LiquidityError;
use crate::{
    libraries::liquidity_math,
    states::{pool::LpPoolStateShape, tick::TickArrayState, Position},
};

#[derive(Accounts)]
pub struct DecreaseLiquidity<'info> {
    //signer
    #[account(mut)]
    pub signer: Signer<'info>,

    //figure the issue
    #[account(mut)]
    pub pool_state_account: Account<'info, LpPoolStateShape>,

    //token program
    pub token_program: Interface<'info, TokenInterface>,

    //mint for the tokens
    pub token_0_mint: InterfaceAccount<'info, Mint>,
    pub token_1_mint: InterfaceAccount<'info, Mint>,

    //user accounts
    #[account(mut, token::mint = token_0_mint, token::authority = signer)]
    pub user_token_0_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, token::mint = token_1_mint, token::authority = signer)]
    pub user_token_1_account: InterfaceAccount<'info, TokenAccount>,

    //vaults account
    #[account(mut, token::mint = token_0_mint, token::authority = pool_state_account)]
    pub token_0_vault_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, token::mint = token_1_mint, token::authority = pool_state_account)]
    pub token_1_vault_account: InterfaceAccount<'info, TokenAccount>,

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

impl<'info> DecreaseLiquidity<'info> {
    pub fn decrease_liquidity(mut self, liquidity_delta: u128) -> Result<()> {
        //read from the states
        let current_tick = self.pool_state_account.tick_current;
        let lower_position = self.position.tick_lower;
        let upper_position = self.position.tick_upper;
        let liquidity_current = self.position.liquidity;
        let sqrt_price_lower = get_sqrt_price_at_tick(self.position.tick_lower);
        let sqrt_price_upper = get_sqrt_price_at_tick(self.position.tick_upper);
        let sqrt_price_current = self.pool_state_account.sqrt_price_x64;

        Ok(())
    }
}
