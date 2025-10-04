#![allow(unexpected_cfgs, deprecated)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("7VFcYgRKbENQYwS5BsanHTBYRcA6xF7iA7CGXrsFZC96");

#[program]
pub mod token_lottery {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        start_time: u64,
        end_time: u64,
        ticket_price: u64,
    ) -> Result<()> {
        ctx.accounts
            .init_config(start_time, end_time, ticket_price, ctx.bumps.lottery)
    }

    pub fn initialize_lottery(ctx: Context<InitializeLottery>) -> Result<()> {
        let collection_mint_bumps = ctx.bumps.collection_mint;
        ctx.accounts.init_lottery(collection_mint_bumps)
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>) -> Result<()> {
        let collection_mint_bumps = ctx.bumps.collection_mint;
        ctx.accounts.buy_lottery_tickets(collection_mint_bumps)
    }

    pub fn commit_randomness(ctx: Context<CommitRandomness>) -> Result<()> {
        ctx.accounts.commit_randomness()
    }

    pub fn reveal_winner(ctx: Context<RevealWinner>) -> Result<()> {
        ctx.accounts.reveal_lottery_winner()
    }
    pub fn claim_wins(ctx: Context<ClaimWinnings>) -> Result<()> {
        ctx.accounts.claim_lottery_win()
    }
}
