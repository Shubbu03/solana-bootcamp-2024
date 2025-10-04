use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Journal {
    pub owner: Pubkey,
    #[max_len(32)]
    pub title: String,
    #[max_len(100)]
    pub message: String,
}
