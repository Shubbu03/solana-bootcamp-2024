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
pub struct Withdraw<'info> {
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
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw_asset(&mut self, amount: u64, bank_token_acc_bumps: u8) -> Result<()> {
        let user = &mut self.user_account;

        let deposited_value: u64;
        if self.mint.to_account_info().key() == user.usdc_address {
            deposited_value = user.deposited_usdc;
        } else {
            deposited_value = user.deposited_sol;
        }

        let time_difference = user.last_updated - Clock::get()?.unix_timestamp;

        let bank = &mut self.bank;

        bank.total_deposits = (bank.total_deposits as f64
            * E.powf(bank.interest_rate as f32 * time_difference as f32) as f64)
            as u64;

        let value_per_share = bank.total_deposits as f64 / bank.total_deposits_shares as f64;

        let user_value = deposited_value as f64 / value_per_share;

        if user_value < amount as f64 {
            return Err(LendingError::InsufficientFunds.into());
        }

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.bank_token_account.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.user_token_account.to_account_info(),
            authority: self.bank_token_account.to_account_info(),
        };

        let mint_key = self.mint.key();

        let signer_seeds: &[&[&[u8]]] =
            &[&[b"treasury", mint_key.as_ref(), &[bank_token_acc_bumps]]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_ctx, amount, self.mint.decimals)?;

        let bank = &mut self.bank;
        let shares_to_remove =
            (amount as f64) / bank.total_deposits as f64 * bank.total_deposits_shares as f64;

        let user = &mut self.user_account;

        if self.mint.to_account_info().key() == user.usdc_address {
            user.deposited_usdc -= amount;
            user.deposited_usdc_shares -= shares_to_remove as u64;
        } else {
            user.deposited_sol -= amount;
            user.deposited_sol_shares -= shares_to_remove as u64;
        }

        bank.total_deposits -= amount;
        bank.total_deposits_shares -= shares_to_remove as u64;

        Ok(())
    }
}
