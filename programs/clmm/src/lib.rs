use anchor_lang::prelude::*;
use anchor_spl::prelude::*;

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

#[derive(Accounts)]
//initialization struct
pub struct Initialize<'info> {
    //signer of the pool
    pub signer: Signer<'info>,
    // make the pool state
    #[accounts(init, payer = signer)]
    //make the vaults
    //mints of the vaults
    //mint for the lp pool
}

#[derive(Accounts)]
//provide lp
pub struct ProvideLp<'info> {}

#[derive(Accounts)]
//swap
pub struct Swap<'info> {}

#[derive(Accounts)]
//removelp
pub struct Removelp<'info> {}
