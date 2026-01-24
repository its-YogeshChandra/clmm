use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

pub mod errors;
pub mod instructions;
pub mod libraries;
pub mod states;

use instructions::decrease_liquidity::DecreaseLiquidity;
use instructions::increase_liquidity::IncreaseLiquidity;
use instructions::initialize_pool::Initialize;
use instructions::open_position::OpenPosition;
use instructions::swap::Swap;

declare_id!("CicZMzrBxTazWhSXGKXkkbnRiYFXSXm2Pe47RvW1X3qt");

#[program]
pub mod clmm {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Initializing CLMM pool");
        Ok(())
    }

    pub fn open_position(
        ctx: Context<OpenPosition>,
        tick_lower: i32,
        tick_upper: i32,
    ) -> Result<()> {
        ctx.accounts.handler(tick_lower, tick_upper)
    }

    pub fn increase_liquidity(
        ctx: Context<IncreaseLiquidity>,
        liquidity: u128,
        amount_0_max: u64,
        amount_1_max: u64,
    ) -> Result<()> {
        ctx.accounts.increase_liquidity(liquidity, amount_0_max, amount_1_max)
    }

    pub fn decrease_liquidity(
        ctx: Context<DecreaseLiquidity>,
        liquidity_delta: u128,
    ) -> Result<()> {
        ctx.accounts.decrease_liquidity(liquidity_delta)
    }

    pub fn swap(
        ctx: Context<Swap>,
        amount_in: u64,
        minimum_amount_out: u64,
        sqrt_price_limit_x64: u128,
    ) -> Result<()> {
        ctx.accounts.swap(amount_in, minimum_amount_out, sqrt_price_limit_x64)
    }
}
