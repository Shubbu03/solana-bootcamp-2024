#![allow(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;
use instructions::*;

mod constants;
mod error;
mod instructions;
mod state;
declare_id!("CGcqN95YmRuWobDvG1YANh3XE8gtecVfvTnKrBNatD3y");

#[program]
pub mod lending {
    use super::*;

    pub fn init_bank(
        ctx: Context<InitBank>,
        liquadation_threshold: u64,
        max_ltv: u64,
    ) -> Result<()> {
        ctx.accounts.init_bank(liquadation_threshold, max_ltv)
    }

    pub fn init_user(ctx: Context<InitUser>, usdc_address: Pubkey) -> Result<()> {
        ctx.accounts.init_user(usdc_address)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit_asset(amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let bank_token_acc_bumps = ctx.bumps.bank_token_account;
        ctx.accounts.withdraw_asset(amount, bank_token_acc_bumps)
    }

    pub fn borrow(ctx: Context<Borrow>, amount: u64) -> Result<()> {
        let bank_token_acc_bumps = ctx.bumps.bank_token_account;
        ctx.accounts.borrow_assets(amount, bank_token_acc_bumps)
    }

    pub fn repay(ctx: Context<Repay>, amount: u64) -> Result<()> {
        ctx.accounts.repay_loan(amount)
    }

    pub fn liquidate(ctx: Context<Liquidate>) -> Result<()> {
        let collateral_bank_token_account = ctx.bumps.collateral_bank_token_account;
        ctx.accounts.liquidate_pool(collateral_bank_token_account)
    }
}
