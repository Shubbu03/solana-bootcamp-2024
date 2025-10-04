use std::f32::consts::E;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{
    error::LendingError,
    state::{Bank, User},
};

#[derive(Accounts)]
pub struct Repay<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [mint.key().as_ref()],
        bump
    )]
    pub bank: Account<'info, Bank>,

    #[account(
        mut,
        seeds = [b"treasury", mint.key().as_ref()],
        bump
    )]
    pub bank_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [signer.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, User>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Repay<'info> {
    pub fn repay_loan(&mut self, amount: u64) -> Result<()> {
        let user = &mut self.user_account;

        let borrowed_value: u64;

        match self.mint.to_account_info().key() {
            key if key == user.usdc_address => {
                borrowed_value = user.borrowed_usdc;
            }
            _ => borrowed_value = user.borrowed_sol,
        }

        let time_difference = user.last_updated_borrow - Clock::get()?.unix_timestamp;

        let bank = &mut self.bank;

        bank.total_borrowed += (bank.total_borrowed as f64
            * E.powf(bank.interest_rate as f32 * time_difference as f32) as f64)
            as u64;

        let value_per_share = bank.total_borrowed as f64 / bank.total_borrowed_shares as f64;

        let user_value = borrowed_value / value_per_share as u64;

        if amount > user_value {
            return Err(LendingError::OverPayment.into());
        }

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.user_token_account.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.bank_token_account.to_account_info(),
            authority: self.bank_token_account.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, amount, self.mint.decimals)?;

        let borrow_ratio = amount.checked_div(bank.total_borrowed).unwrap();
        let user_shares = bank
            .total_borrowed_shares
            .checked_mul(borrow_ratio)
            .unwrap();

        match self.mint.to_account_info().key() {
            key if key == user.usdc_address => {
                user.borrowed_usdc -= amount;
                user.borrowed_usdc_shares -= user_shares;
            }
            _ => {
                user.borrowed_sol -= amount;
                user.borrowed_sol_shares -= user_shares;
            }
        }

        bank.total_borrowed -= amount;
        bank.total_borrowed_shares -= user_shares;
        Ok(())
    }
}
