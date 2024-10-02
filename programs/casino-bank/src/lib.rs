use anchor_lang::prelude::*;

declare_id!("99xm6t1bmVaQQe3nH6QTEkRh5mqQnQtCh6jUMqnXuyMe");

#[program]
pub mod casino_bank {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
