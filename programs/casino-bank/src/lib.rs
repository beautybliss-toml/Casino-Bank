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
    declare_id!("3TuMY8F5pbeUXwbQymm4Vbi1X3ntx1vQ2dt3cqZWbrv7");
}

#[program]
pub mod casino_bank {
    use super::*;

    /// allow user deposit sol to the pda
    /// # Arguments
    ///
    /// * `ctx` - 
    /// * `amount` -
    ///
    pub fn deposit_sol(ctx: Context<DepositSol>, amount: u64) -> Result<()> {
        assert!(amount > 0);

        msg!("Sol amount transfer in: {}!", amount);


        Ok(())
    }

    /// allow owner deposit sol to the pda
    /// # Arguments
    ///
    /// * `ctx` - 
    /// * `amount` -
    ///
    pub fn emergency_sol_withdraw(ctx: Context<WithdrawSol>, amount: u64) -> Result<()> {
        assert!(amount > 0);

        msg!("Sol amount transfer in: {}!", amount);


        Ok(())
    }

    /// transfer sol to user by owner
    /// # Arguments
    ///
    /// * `ctx` - 
    /// * `amount` -
    ///
    pub fn transfer_sol_to_user_by_owner(ctx: Context<TransferSol>, amount: u64) -> Result<()> {
        assert!(amount > 0);

        msg!("Sol amount transfer in: {}!", amount);

        Ok(())
    }

    /// allow user deposit token to the pda
    /// # Arguments
    ///
    /// * `ctx` - 
    /// * `token_amount` -
    ///
    pub fn deposit_token_to_pda(ctx: Context<DepositToken>, token_amount: u64) -> Result<()> {
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

        emit!(DepositTokenEvent{
            sender_account: *(ctx.accounts.sender_token_account.to_account_info().key),
            token: *(ctx.accounts.mint_of_token_being_sent.to_account_info().key),
            amount: token_amount
        });

        Ok(())
    }

    /// allow owner withdraw token from pda
    ///
    /// # Arguments
    ///
    /// * `ctx` -
    /// * `token_amount` -
    ///
    pub fn emergency_token_withdraw(ctx: Context<WithdrawToken>, token_amount: u64) -> Result<()> {
        // check admin
        assert!(*(ctx.accounts.signer.to_account_info().key) == crate::admin::id());
        // check balance
        assert!(ctx.accounts.vault_token_account.amount >= token_amount);

        msg!("Token amount transfer out: {}!", token_amount);

        // make transfer
        let transfer_instruction = Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.signer_ata.to_account_info(),
            authority: ctx.accounts.token_account_owner_pda.to_account_info(),
        };
        // make signer of pda
        let bump = ctx.bumps.token_account_owner_pda;
        let seeds = &[b"token_account_owner_pda".as_ref(), &[bump]];
        let signer = &[&seeds[..]];
        // cpi
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            signer,
        );
        anchor_spl::token::transfer(cpi_ctx, token_amount)?;
        // emit event
        emit!(WithdrawTokenEvent{
            owner_account: *(ctx.accounts.signer.to_account_info().key),
            token: *(ctx.accounts.mint_of_token_being_sent.to_account_info().key),
            amount: token_amount
        });
        Ok(())
    }

    /// transfer token to user from pda by owner
    ///
    /// # Arguments
    ///
    /// * `ctx` -
    /// * `token_amount` -
    ///
    pub fn transfer_token_to_user_by_owner(ctx: Context<TransferToken>, token_amount: u64) -> Result<()> {
        // check admin
        assert!(*(ctx.accounts.signer.to_account_info().key) == crate::admin::id());
        // check balance
        assert!(ctx.accounts.vault_token_account.amount >= token_amount);

        msg!("Token amount transfer out: {}!", token_amount);

        // make transfer
        let transfer_instruction = Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.signer_ata.to_account_info(),
            authority: ctx.accounts.token_account_owner_pda.to_account_info(),
        };
        
        Ok(())
    }

}

#[derive(Accounts)]
pub struct DepositSol<'info> {
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
    
    #[account(mut,
        constraint = signer_ata.mint == *(mint_of_token_being_sent.to_account_info().key) && signer_ata.owner == *(signer.to_account_info().key)
    )]
    signer_ata: Account<'info, TokenAccount>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>
}
#[derive(Accounts)]
pub struct WithdrawSol<'info> {
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
    
    #[account(mut,
        constraint = signer_ata.mint == *(mint_of_token_being_sent.to_account_info().key) && signer_ata.owner == *(signer.to_account_info().key)
    )]
    signer_ata: Account<'info, TokenAccount>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>
}
#[derive(Accounts)]
pub struct TransferSol<'info> {
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
    
    #[account(mut,
        constraint = signer_ata.mint == *(mint_of_token_being_sent.to_account_info().key) && signer_ata.owner == *(signer.to_account_info().key)
    )]
    signer_ata: Account<'info, TokenAccount>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct DepositToken<'info> {
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
pub struct WithdrawToken<'info> {
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
    
    #[account(mut,
        constraint = signer_ata.mint == *(mint_of_token_being_sent.to_account_info().key) && signer_ata.owner == *(signer.to_account_info().key)
    )]
    signer_ata: Account<'info, TokenAccount>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
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
    
    #[account(mut,
        constraint = signer_ata.mint == *(mint_of_token_being_sent.to_account_info().key) && signer_ata.owner == *(signer.to_account_info().key)
    )]
    signer_ata: Account<'info, TokenAccount>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>
}


#[account]
pub struct TokenAccOwnerAccount {}

