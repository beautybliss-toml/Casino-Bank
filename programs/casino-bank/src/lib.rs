pub mod event;
pub mod error;

use anchor_lang::prelude::*;
use anchor_spl::token::Transfer;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::error::ErrorCode;
use crate::event::*;

declare_id!("99xm6t1bmVaQQe3nH6QTEkRh5mqQnQtCh6jUMqnXuyMe");

pub mod admin {
    use anchor_lang::prelude::declare_id;
    #[cfg(feature = "devnet")]
    declare_id!("adMCyoCgfkg7bQiJ9aBJ59H3BXLY3r5LNLfPpQfMzBe");
    #[cfg(not(feature = "devnet"))]
    declare_id!("3TuMY8F5pbeUXwbQymm4Vbi1X3ntx1vQ2dt3cqZWbrv7");
}

#[program]
pub mod casino_bank {
    use super::*;

    /// Allow user deposit token to the PDA
    /// # Arguments
    ///
    /// * `ctx` - 
    /// * `token_amount` -
    ///
    pub fn deposit_spl_to_pda(ctx: Context<DepositSpl>, token_amount: u64) -> Result<()> {
        assert!(token_amount > 0);

        msg!("Token amount transfer in: {}!", token_amount);

        let transfer_instruction = Transfer {
            from: ctx.accounts.sender_token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
        );

        anchor_spl::token::transfer(cpi_ctx, token_amount)?;

        emit!(DepositSplEvent{
            sender_account: *(ctx.accounts.sender_token_account.to_account_info().key),
            token: *(ctx.accounts.mint_of_token_being_sent.to_account_info().key),
            amount: token_amount
        });

        Ok(())
    }

}

#[derive(Accounts)]
pub struct DepositSpl<'info> {
    #[account(
        init_if_needed,
        payer = signer,
        seeds=[b"token_account_owner_pda"],
        space = 8,
        bump
    )]
    token_account_owner_pda: Account<'info, TokenAccOwnerAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        seeds=[b"token_vault", mint_of_token_being_sent.key().as_ref()],
        token::mint=mint_of_token_being_sent,
        token::authority=token_account_owner_pda,
        bump
    )]
    vault_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    sender_token_account: Account<'info, TokenAccount>,

    mint_of_token_being_sent: Account<'info, Mint>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct WithdrawSpl<'info> {
    #[account(mut,
        seeds=[b"token_account_owner_pda"],
        bump
    )]
    token_account_owner_pda: Account<'info, TokenAccOwnerAccount>,

    #[account(mut,
        seeds=[b"token_vault", mint_of_token_being_sent.key().as_ref()],
        bump,
        token::mint=mint_of_token_being_sent,
        token::authority=token_account_owner_pda,
    )]
    vault_token_account: Account<'info, TokenAccount>,

    mint_of_token_being_sent: Account<'info, Mint>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>
}

#[account]
pub struct TokenAccOwnerAccount {}

