use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct DepositEvent {
    #[index]
    pub sender_account: Pubkey,
    pub amount: u64,
}

#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct WithdrawEvent {
    #[index]
    pub owner_account: Pubkey,
    pub amount: u64,
}

#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct DepositTokenEvent {
    #[index]
    pub sender_account: Pubkey,
    pub token: Pubkey,
    pub amount: u64,
}

#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct WithdrawTokenEvent {
    #[index]
    pub owner_account: Pubkey,
    pub token: Pubkey,
    pub amount: u64,
}

#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct ProcessedWithdrawEvent {
    
}