use anchor_lang::prelude::*;

use crate::{Candidate, Poll};

#[derive(Accounts)]
#[instruction(candidate_name: String, poll_id: u64)]
pub struct Vote<'info> {
    pub user: Signer<'info>,

    #[account(
        seeds = [candidate_name.as_bytes(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub candidate: Account<'info, Candidate>,

    #[account(
        seeds = [b"poll", poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,
}

impl<'info> Vote<'info> {
    pub fn vote(&mut self) -> Result<()> {
        self.candidate.candidate_vote += 1;
        Ok(())
    }
}
