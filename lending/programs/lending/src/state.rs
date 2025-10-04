use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User {
    pub owner: Pubkey,
    pub deposited_sol: u64,
    pub deposited_sol_shares: u64,
    pub borrowed_sol: u64,
    pub borrowed_sol_shares: u64,
    pub deposited_usdc: u64,
    pub deposited_usdc_shares: u64,
    pub borrowed_usdc: u64,
    pub borrowed_usdc_shares: u64,
    pub usdc_address: Pubkey,
    pub last_updated: i64,
    pub last_updated_borrow: i64,
}

#[account]
#[derive(InitSpace)]
pub struct Bank {
    pub authority: Pubkey,
    pub mint_address: Pubkey,
    pub total_deposits: u64,
    pub total_deposits_shares: u64,
    pub total_borrowed: u64,
    /// Current number of borrowed shares in the bank
    pub total_borrowed_shares: u64,
    pub liquadation_threshold: u64, // loan to value at which a loan is defined a under collatorised & can be liquidated
    pub liquadation_bonus: u64, // % of liquidation that will be sent to the liquidator as a bonus to process the liquidation
    pub liquadation_close_factor: u64, // % of collateral that can be liquidated
    pub max_ltv: u64,           // max % of collateral that can be borrowed for an asset
    pub last_updated: i64,
    pub interest_rate: u64,
}
