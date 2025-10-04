#![allow(unexpected_cfgs, deprecated)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("8L3PKN3SPwKTr9aMseFtXdF9errU6JcuZ2siHNxK9PFp");

#[program]
pub mod voting_program {
    use super::*;

    pub fn initialize_poll(
        ctx: Context<InitializePoll>,
        poll_id: u64,
        description: String,
        poll_start: u64,
        poll_end: u64,
    ) -> Result<()> {
        ctx.accounts
            .init_poll(poll_id, description, poll_start, poll_end)
    }

    pub fn initialize_candidate(
        ctx: Context<InitializeCandidate>,
        candidate_name: String,
        poll_id: u64,
    ) -> Result<()> {
        ctx.accounts.init_candidate(candidate_name, poll_id)
    }
    pub fn vote(ctx: Context<Vote>, _candidate_name: String, _poll_id: u64) -> Result<()> {
        ctx.accounts.vote()
    }
}

//signature -> 5gur7V9U1cHzRUsaT57mqCnzvmsarNe9VJyHoqWrUnGSTvEdhuubXegHWFvi8WYsS6oTE1qbsU8QpLbC8yCw7dro