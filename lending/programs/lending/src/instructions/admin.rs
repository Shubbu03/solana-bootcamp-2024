use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::state::{Bank, User};

#[derive(Accounts)]
pub struct InitBank<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        space = 8 + Bank::INIT_SPACE,
        seeds = [mint.key().as_ref()],
        bump
    )]
    pub bank: Account<'info, Bank>,

    #[account(
        init,
        payer = signer,
        token::mint = mint,
        token::authority = bank_token_account,
        seeds = [b"treasury", mint.key().as_ref()],
        bump
    )]
    pub bank_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + User::INIT_SPACE,
        seeds = [signer.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, User>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitBank<'info> {
    pub fn init_bank(&mut self, liquadation_threshold: u64, max_ltv: u64) -> Result<()> {
        self.bank.set_inner(Bank {
            authority: self.signer.key(),
            mint_address: self.mint.key(),
            total_deposits: 0,
            total_deposits_shares: 0,
            total_borrowed: 0,
            total_borrowed_shares: 0,
            liquadation_threshold,
            liquadation_bonus: 0,
            liquadation_close_factor: 0,
            max_ltv,
            last_updated: Clock::get()?.unix_timestamp,
            interest_rate: 0.05 as u64,
        });
        Ok(())
    }
}

impl<'info> InitUser<'info> {
    pub fn init_user(&mut self, usdc_address: Pubkey) -> Result<()> {
        self.user_account.set_inner(User {
            owner: self.signer.key(),
            deposited_sol: 0,
            deposited_sol_shares: 0,
            borrowed_sol: 0,
            borrowed_sol_shares: 0,
            deposited_usdc: 0,
            deposited_usdc_shares: 0,
            borrowed_usdc: 0,
            borrowed_usdc_shares: 0,
            usdc_address,
            last_updated: Clock::get()?.unix_timestamp,
            last_updated_borrow: Clock::get()?.unix_timestamp,
        });
        Ok(())
    }
}
