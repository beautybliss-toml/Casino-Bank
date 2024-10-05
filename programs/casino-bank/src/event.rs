use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct DepositSolEvent {
    #[index]
    pub sender_account: Pubkey,
    pub amount: u64,
}

#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct WithdrawSolEvent {
    #[index]
    pub owner_account: Pubkey,
    pub amount: u64,
}

#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct DepositSplEvent {
    #[index]
    pub sender_account: Pubkey,
    pub token: Pubkey,
    pub amount: u64,
}

#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct WithdrawSplEvent {
    #[index]
    pub owner_account: Pubkey,
    pub token: Pubkey,
    pub amount: u64,
}

#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct ProcessedWithdrawEvent {
    
}