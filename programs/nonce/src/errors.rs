use anchor_lang::prelude::*;

#[error_code]
pub enum NonceErrror{
    #[msg("Savings account is inactive")]
    SavingsInactive,
    #[msg("Funds are still Locked")]
    FundsStillLocked,
    #[msg("Unauthorized access to savings Account")]
    Unauthorized,
    #[msg("Insufficient Funds")]
    InsufficientFunds
}