use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("99xm6t1bmVaQQe3nH6QTEkRh5mqQnQtCh6jUMqnXuyMe");

#[program]
pub mod casino_bank {
    use super::*;

    /// Initialize
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    /// Allow user deposit token to the contract account
    /// # Arguments
    ///
    /// * `ctx` - 
    /// * `token_address` - 
    /// * `token_amount` -
    ///
    pub fn deposit(ctx: Context<Deposit>, token_address: Pubkey, token_amount: u64) -> Result<()> {
        Ok(())
    }

    /// Allow owner withdraw any token from the contract account
    ///
    /// # Arguments
    ///
    /// * `ctx` -
    /// * `token_address` - 
    /// * `token_amount` -
    ///
    pub fn emergency_withdraw(ctx: Context<Withdraw>, token_address: Pubkey, token_amount: u64) -> Result<()> {
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_if_needed,
        payer = signer,
        seeds = [b"token_vault"],
        bump,
        space = 1000
    )]
    token_vault_account: Account<'info, TokenAccount>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        seeds = [b"token_vault"],
        bump
    )]
    token_vault_account: Account<'info, TokenAccount>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct Withdraw {

}

#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct DepositEvent {

}

#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct WithdrawEvent {

}

#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct ProcessWithdrawEvent {

}