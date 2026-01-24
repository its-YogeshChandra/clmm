use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked};

use crate::errors::SwapError;
use crate::libraries::swap_math::{
    compute_swap_step, find_next_initialized_tick, get_amount_0_delta, get_amount_1_delta, Q64,
};
use crate::libraries::tick_math::{get_sqrt_price_at_tick, get_tick_at_sqrt_price};
use crate::states::{pool::LpPoolStateShape, tick::TickArrayState};

pub const MIN_SQRT_PRICE_X64: u128 = 4295048016;
pub const MAX_SQRT_PRICE_X64: u128 = 79226673521066979257578248091;

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub pool_state: Account<'info, LpPoolStateShape>,

    pub token_program: Interface<'info, TokenInterface>,

    /// The mint of input token
    pub input_mint: InterfaceAccount<'info, Mint>,

    /// The mint of output token
    pub output_mint: InterfaceAccount<'info, Mint>,

    /// User's input token account
    #[account(mut, token::mint = input_mint, token::authority = signer)]
    pub user_input_account: InterfaceAccount<'info, TokenAccount>,

    /// User's output token account
    #[account(mut, token::mint = output_mint, token::authority = signer)]
    pub user_output_account: InterfaceAccount<'info, TokenAccount>,

    /// Pool's input vault
    #[account(mut, token::authority = pool_state)]
    pub input_vault: InterfaceAccount<'info, TokenAccount>,

    /// Pool's output vault
    #[account(mut, token::authority = pool_state)]
    pub output_vault: InterfaceAccount<'info, TokenAccount>,

    /// Tick array containing current tick
    #[account(mut)]
    pub tick_array: AccountLoader<'info, TickArrayState>,
}

/// State during swap execution
struct SwapState {
    amount_remaining: u64,
    amount_calculated: u64,
    sqrt_price_x64: u128,
    tick: i32,
    liquidity: u128,
    fee_growth_global: u128,
}

impl<'info> Swap<'info> {
    pub fn swap(
        &mut self,
        amount_in: u64,
        minimum_amount_out: u64,
        sqrt_price_limit_x64: u128,
    ) -> Result<()> {
        let pool = &mut self.pool_state;

        // Determine swap direction based on input mint
        let zero_for_one = self.input_mint.key() == pool.token_0_mint;

        // Validate sqrt_price_limit
        if zero_for_one {
            require!(
                sqrt_price_limit_x64 < pool.sqrt_price_x64
                    && sqrt_price_limit_x64 >= MIN_SQRT_PRICE_X64,
                SwapError::InvalidSqrtPriceLimit
            );
        } else {
            require!(
                sqrt_price_limit_x64 > pool.sqrt_price_x64
                    && sqrt_price_limit_x64 <= MAX_SQRT_PRICE_X64,
                SwapError::InvalidSqrtPriceLimit
            );
        }

        require!(pool.liquidity > 0, SwapError::NoLiquidity);
        require!(amount_in > 0, SwapError::ZeroAmount);

        // Initialize swap state
        let mut state = SwapState {
            amount_remaining: amount_in,
            amount_calculated: 0,
            sqrt_price_x64: pool.sqrt_price_x64,
            tick: pool.tick_current,
            liquidity: pool.liquidity,
            fee_growth_global: if zero_for_one {
                pool.fee_growth_global_0
            } else {
                pool.fee_growth_global_1
            },
        };

        // Load tick array
        let mut tick_array_state = self.tick_array.load_mut()?;
        let tick_spacing = pool.tick_spacing as i32;

        // Main swap loop
        while state.amount_remaining > 0 && state.sqrt_price_x64 != sqrt_price_limit_x64 {
            // Find next initialized tick
            let next_tick_result = find_next_initialized_tick(
                &tick_array_state.ticks,
                tick_array_state.start_tick_index,
                state.tick,
                tick_spacing,
                zero_for_one,
            );

            let (next_tick, tick_index_in_array, is_initialized) = match next_tick_result {
                Some(result) => result,
                None => break,
            };

            // Get sqrt_price at next tick
            let sqrt_price_next_tick = get_sqrt_price_at_tick(next_tick);

            // Determine target price (capped by limit)
            let sqrt_price_target = if zero_for_one {
                sqrt_price_next_tick.max(sqrt_price_limit_x64)
            } else {
                sqrt_price_next_tick.min(sqrt_price_limit_x64)
            };

            // Compute swap step
            let (sqrt_price_next, amount_in_step, amount_out_step, fee_amount) = compute_swap_step(
                state.sqrt_price_x64,
                sqrt_price_target,
                state.liquidity,
                state.amount_remaining,
                pool.fee_rate,
                zero_for_one,
            );

            // Update amounts
            state.amount_remaining = state
                .amount_remaining
                .saturating_sub(amount_in_step + fee_amount);
            state.amount_calculated += amount_out_step;

            // Update fee growth
            if state.liquidity > 0 && fee_amount > 0 {
                let fee_growth_delta = (fee_amount as u128 * Q64) / state.liquidity;
                state.fee_growth_global += fee_growth_delta;
            }

            // Update price
            state.sqrt_price_x64 = sqrt_price_next;

            // Handle tick crossing
            if sqrt_price_next == sqrt_price_next_tick && is_initialized {
                let tick_state = &mut tick_array_state.ticks[tick_index_in_array];

                // Flip fee growth outside
                tick_state.fee_growth_outside_0 =
                    pool.fee_growth_global_0 - tick_state.fee_growth_outside_0;
                tick_state.fee_growth_outside_1 =
                    pool.fee_growth_global_1 - tick_state.fee_growth_outside_1;

                // Update liquidity
                let liquidity_net = tick_state.liquidity_net;
                if zero_for_one {
                    // Going left, subtract liquidity_net
                    if liquidity_net > 0 {
                        state.liquidity = state.liquidity.saturating_sub(liquidity_net as u128);
                    } else {
                        state.liquidity = state.liquidity + ((-liquidity_net) as u128);
                    }
                } else {
                    // Going right, add liquidity_net
                    if liquidity_net > 0 {
                        state.liquidity = state.liquidity + (liquidity_net as u128);
                    } else {
                        state.liquidity = state.liquidity.saturating_sub((-liquidity_net) as u128);
                    }
                }

                // Update tick
                state.tick = if zero_for_one {
                    next_tick - 1
                } else {
                    next_tick
                };
            } else {
                // Price didn't reach tick, calculate new tick from price
                state.tick = get_tick_at_sqrt_price(state.sqrt_price_x64);
            }
        }

        drop(tick_array_state);

        // Calculate final amounts
        let amount_in_used = amount_in - state.amount_remaining;
        let amount_out = state.amount_calculated;

        // Slippage check
        require!(
            amount_out >= minimum_amount_out,
            SwapError::TooLittleOutputReceived
        );

        // Update pool state
        pool.sqrt_price_x64 = state.sqrt_price_x64;
        pool.tick_current = state.tick;
        pool.liquidity = state.liquidity;
        if zero_for_one {
            pool.fee_growth_global_0 = state.fee_growth_global;
        } else {
            pool.fee_growth_global_1 = state.fee_growth_global;
        }

        // Transfer input tokens: User -> Vault
        let cpi_accounts_in = TransferChecked {
            from: self.user_input_account.to_account_info(),
            to: self.input_vault.to_account_info(),
            authority: self.signer.to_account_info(),
            mint: self.input_mint.to_account_info(),
        };
        let cpi_ctx_in = CpiContext::new(self.token_program.to_account_info(), cpi_accounts_in);
        token_interface::transfer_checked(cpi_ctx_in, amount_in_used, self.input_mint.decimals)?;

        // Transfer output tokens: Vault -> User (PDA signer)
        let pool_bump = self.pool_state.bump;
        let token_0_key = self.pool_state.token_0_mint;
        let token_1_key = self.pool_state.token_1_mint;
        let seeds: &[&[u8]] = &[
            b"pool_state_v1",
            token_0_key.as_ref(),
            token_1_key.as_ref(),
            &[pool_bump],
        ];
        let signer_seeds = &[seeds];

        let cpi_accounts_out = TransferChecked {
            from: self.output_vault.to_account_info(),
            to: self.user_output_account.to_account_info(),
            authority: self.pool_state.to_account_info(),
            mint: self.output_mint.to_account_info(),
        };
        let cpi_ctx_out = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts_out,
            signer_seeds,
        );
        token_interface::transfer_checked(cpi_ctx_out, amount_out, self.output_mint.decimals)?;

        Ok(())
    }
}
