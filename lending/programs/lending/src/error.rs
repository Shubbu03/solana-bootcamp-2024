use anchor_lang::prelude::*;

#[error_code]
pub enum LendingError {
    #[msg("Insufficient Funds")]
    InsufficientFunds,
    #[msg("Over Borrowable Amount")]
    OverBorrowableAmount,
    #[msg("Over Payment done")]
    OverPayment,
    #[msg("Not Undercollateralized")]
    NotUndercollateralized,
}
