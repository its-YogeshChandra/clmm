use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Token, TokenAccount},
    token_interface::{self, Burn, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked},
};

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

#[account]
#[derive(InitSpace)]
pub struct LpPoolStateShape {
    //mint of both vauts
    pub usdc_mint: Pubkey,
    pub wsol_mint: Pubkey,
    //vault addresses
    pub usdc_vault_address: Pubkey,
    pub sol_vault_address: Pubkey,
    //mint
    pub lpmint: Pubkey,
    //bump
    bump: u8,
}

#[derive(Accounts)]
//initialization struct
pub struct Initialize<'info> {
    //signer of the pool
    #[account(mut)]
    pub signer: Signer<'info>,

    pub usdc_mint: InterfaceAccount<'info, Mint>,
    pub sol_mint: InterfaceAccount<'info, Mint>,

    //token program
    pub token_program: Interface<'info, Token>,

    //system program
    pub system_program: Program<'info, System>,

    //lp token_mint
    #[account(mut)]
    pub lp_token_mint: InterfaceAccount<'info, Mint>,

    // make the pool state
    #[accounts(init, payer = signer, space= 8+LpPoolStateShape::INIT_SPACE, seeds = [b"pool_state_v1", usdc_mint.key().as_ref(), wsol_mint.key().as_ref()], bump)]
    pub pool_state_account: Account<'info, LpPoolStateShape>,

    //make the vaults
    #[accounts(init, payer=signer, token::mint = usdc_mint, token::authority = pool_state_account, token::token_program, seeds = [b"usdc_vault", usdc_mint.key().as_ref()], bump)]
    pub usdc_vault_account: InterfaceAccount<'info, TokenAccount>,

    #[accounts(init, payer=signer, token::mint = usdc_mint, token::authority = pool_state_account, token::token_program, seeds = [b"wsol_vault", wsol_mint.key().as_ref()], bump)]
    pub wsol_vault_account: InterfaceAccount<'info, TokenAccount>,

    //mint for the lp tokens
    #[account(init, payer = signer, mint::decimals= 9, mint::authority = pool_state_account, mint::freeze_authority = pool_state_account)]
    pub lp_token_mint: InterfaceAccount<'info, Mint>,
}

#[error_code]
pub enum ProvideLpError {
    #[msg("multiplication error")]
    MathOverflowError,

    #[msg("liquidity too low")]
    LiquidityTooLow,
}

#[derive(Accounts)]
//provide lp
pub struct ProvideLp<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    //token program
    pub token_program: Interface<'info, TokenInterface>,

    //system program
    pub system_program: Progam<'info, System>,

    //pool_state_account
    pub pool_state_account: Account<'info, LpPoolStateShape>,

    //mints for the program
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    pub wsol_mint: InterfaceAccount<'info, Mint>,
    pub lp_token_mint: InterfaceAccount<'info, Mint>,

    #[account(mut, token::mint = usdc_mint, token::authority = pool_state_account )]
    pub usdc_vault_address: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, token::mint = wsol_mint, token::authority = pool_state_account )]
    pub wsol_vault_address: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, token::authority = signer, token::mint = usdc_mint)]
    pub user_usdc_acccount: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, token::authority = signer, token::mint = wsol_mint)]
    pub user_wsol_acccount: InterfaceAccount<'info, TokenAccount>,

    //make the ata for the user
    #[account(init, payer = signer , token::mint = lp_token_mint, token::authority = signer, token::token_program = token_program)]
    pub user_lpata: InterfaceAccount<'info, TokenAccount>,
}

//swap function
#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    //token program
    token_program: Interface<'info, TokenInterface>,

    //pool_state_account
    pub pool_state_account: Account<'info, LpPoolStateShape>,

    #[account(mut, token::authority = pool_state_account )]
    pub input_vault_address: InterfaceAccount<'info, TokenAccount>,
    #[account(mut,token::authority = pool_state_account )]
    pub output_vault_address: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, token::authority = signer,)]
    pub user_input_acccount: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, token::authority = signer,)]
    pub user_output_acccount: InterfaceAccount<'info, TokenAccount>,
}

//error code for swap

//functions for swap
impl<'info> Swap<'info> {}

//remove lp
#[derive(Accounts)]
pub struct Removelp<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    //token program
    pub token_program: Interface<'info, TokenInterface>,

    //pool_state_account
    pub pool_state_account: Account<'info, LpPoolStateShape>,

    //mint accounts
    pub lp_token_mint: InterfaceAccount<'info, Mint>,
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    pub wsol_mint: InterfaceAccount<'info, Mint>,

    //vault address
    #[account(mut, token::mint = usdc_mint ,token::authority = pool_state_account )]
    pub usdc_vault_address: InterfaceAccount<'info, TokenAccount>,
    #[account(mut,token::mint = wsol_mint , token::authority = pool_state_account )]
    pub wsol_vault_address: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, token::authority = signer, token::mint= usdc_mint)]
    pub user_usdc_acccount: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, token::authority = signer, token::mint = wsol_mint)]
    pub user_wsol_acccount: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub user_lp_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> Removelp<'info> {}
