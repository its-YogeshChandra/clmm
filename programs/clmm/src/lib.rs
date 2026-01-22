use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self},
    token_interface::{self, Burn, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked},
};

mod errors;
mod instructions;
mod libraries;
mod states;

declare_id!("CicZMzrBxTazWhSXGKXkkbnRiYFXSXm2Pe47RvW1X3qt");

#[program]
pub mod clmm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn providelp(ctx: Context<ProvideLp>) -> Result<()> {
        Ok(())
    }
}
