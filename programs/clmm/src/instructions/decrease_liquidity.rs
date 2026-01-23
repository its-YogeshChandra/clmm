use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self},
    token_interface::{self, Burn, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{
    errors::LiquidityError,
    libraries::{get_amounts_0_from_liquidity, get_amounts_1_from_liquidity},
    states::position,
};
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

        //validate the conditions
        require!(liquidity_delta > 0, LiquidityError::ZeroLiquidity);
        require!(
            liquidity_delta <= self.position.liquidity,
            LiquidityError::InsufficientLiquidity
        );

        let amount_0: u64;
        let amount_1: u64;
        //calculate token amount to return
        if current_tick < self.position.tick_lower {
            //below range only token 0
            amount_0 =
                get_amounts_0_from_liquidity(sqrt_price_lower, sqrt_price_upper, liquidity_delta);
            amount_1 = 0
        } else if current_tick >= self.position.tick_upper {
            //above range then only token 1
            amount_1 =
                get_amounts_1_from_liquidity(sqrt_price_lower, sqrt_price_upper, liquidity_delta);
            amount_0 = 0
        } else {
            //in range then both the tokens is needed
            amount_0 =
                get_amounts_0_from_liquidity(sqrt_price_current, sqrt_price_upper, liquidity_delta);
            amount_1 =
                get_amounts_1_from_liquidity(sqrt_price_lower, sqrt_price_current, liquidity_delta)
        }

        //load the tick state from the tick array
        let tick_lower_state = self.tick_array_lower.load()?;
        let tick_upper_state = self.tick_array_upper.load()?;
        let lower_start_index = tick_lower_state.start_tick_index;
        let upper_start_index = tick_upper_state.start_tick_index;

        //calculate the index in the array
        let tick_spacing = self.pool_state_account.tick_spacing as i32;
        let lower_tick_index = (lower_position - lower_start_index) / tick_spacing;
        let upper_tick_index = (upper_position - upper_start_index) / tick_spacing;

        //get the tick states
        let lower_tick = tick_lower_state.ticks[lower_tick_index as usize];
        let upper_tick = tick_upper_state.ticks[upper_tick_index as usize];

        //access the fee growth
        let fee_growth_global = self.pool_state_account.fee_growth_global_0;

        //calculat the fee growth inside for token_0
        let fee_growth_below_0 = get_fee_growth_below(
            current_tick,
            lower_position,
            fee_growth_global,
            lower_tick.fee_growth_outside_0,
        );

        let fee_growth_above_0 = get_fee_growth_above(
            current_tick,
            upper_position,
            fee_growth_global,
            upper_tick.fee_growth_outside_0,
        );

        // Calculate fee_growth_inside for token0
        let fee_growth_inside_0 = fee_growth_global - fee_growth_below_0 - fee_growth_above_0;

        // Calculate fee_growth_inside for token1
        let fee_growth_global_1 = self.pool_state_account.fee_growth_global_1;
        let fee_growth_below_1 = get_fee_growth_below(
            current_tick,
            lower_position,
            fee_growth_global_1,
            lower_tick.fee_growth_outside_1,
        );
        let fee_growth_above_1 = get_fee_growth_above(
            current_tick,
            upper_position,
            fee_growth_global_1,
            upper_tick.fee_growth_outside_1,
        );
        let fee_growth_inside_1 = fee_growth_global_1 - fee_growth_below_1 - fee_growth_above_1;

        // Calculate tokens owed using OLD liquidity
        let q64: u128 = 1u128 << 64;
        let tokens_owed_0 = ((fee_growth_inside_0
            .wrapping_sub(self.position.fee_growth_inside_0_last))
            * liquidity_current
            / q64) as u64;
        let tokens_owed_1 = ((fee_growth_inside_1
            .wrapping_sub(self.position.fee_growth_inside_1_last))
            * liquidity_current
            / q64) as u64;

        // Drop immutable borrows before mutable operations
        drop(tick_lower_state);
        drop(tick_upper_state);

        // Update position fee state
        self.position.tokens_owed_0 += tokens_owed_0;
        self.position.tokens_owed_1 += tokens_owed_1;
        self.position.fee_growth_inside_0_last = fee_growth_inside_0;
        self.position.fee_growth_inside_1_last = fee_growth_inside_1; //load tick array

        Ok(())
    }
}
