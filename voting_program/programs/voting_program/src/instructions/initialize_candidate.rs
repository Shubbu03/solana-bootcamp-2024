use anchor_lang::prelude::*;

use crate::{Candidate, Poll};

#[derive(Accounts)]
#[instruction(candidate_name: String, poll_id: u64)]
pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + Candidate::INIT_SPACE,
        seeds = [candidate_name.as_bytes(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub candidate: Account<'info, Candidate>,

    #[account(
        seeds = [b"poll", poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeCandidate<'info> {
    pub fn init_candidate(&mut self, candidate_name: String, _poll_id: u64) -> Result<()> {
        self.candidate.set_inner(Candidate {
            candidate_name,
            candidate_vote: 0
        });

        self.poll.candidate_amount += 1;
        
        Ok(())
    }
}
