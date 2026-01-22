use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self},
    token_interface::{self, Burn, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{
    libraries::{get_sqrt_price_at_tick, liquidity_math},
    states::{
        pool::LpPoolStateShape,
        position,
        tick::{self, TickArrayState},
        Position,
    },
};

#[derive(Accounts)]
pub struct IncreaseLiquidity<'info> {
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
    #[account(mut, token::mint = usdc_mint, token::authority = signer)]
    pub user_token_0_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, token::mint = wsol_mint, token::authority = signer)]
    pub user_token_1_account: InterfaceAccount<'info, TokenAccount>,

    //vaults account
    #[account(mut, token::mint = usdc_mint, token::authority = pool_state_account)]
    pub token_0_vault_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, token::mint = wsol_mint, token::authority = pool_state_account)]
    pub token_1_vault_account: InterfaceAccount<'info, TokenAccount>,

    //position
    //can pass the constrain this way
    #[account(mut, constraint = position.owner==signer.key())]
    pub position: Account<'info, Position>,

    //tick array upper
    //used account loader
    pub tick_array_lower: AccountLoader<'info, TickArrayState>,
    pub tick_array_upper: AccountLoader<'info, TickArrayState>,
}

//handle
impl<'info> IncreaseLiquidity<'info> {
    pub fn increase_liquidity(
        self,
        liquidity: u128,
        token_0_amount: u64,
        token_1_amount: u64,
    ) -> Result<()> {
        //read from the accounts
        let current_price = self.pool_state_account.lpmint;
        let current_tick = self.pool_state_account.tick_current;
        let lower_position = self.position.tick_lower;
        let upper_position = self.position.tick_upper;
        let liquidity_current = self.position.liquidity;
        let sqrt_price_lower = get_sqrt_price_at_tick(self.position.tick_lower);
        let sqrt_price_upper = get_sqrt_price_at_tick(self.position.tick_upper);
        let sqrt_price_current = self.pool_state_account.sqrt_price_x64;

        //right now just single function will segregate this in future
        //there are 3 cases
        //case 1 : current_price is below position range
        if tick_current < tick_lower {
            //get liquidity from account 0 function called
        }

        Ok(())
    }
}
