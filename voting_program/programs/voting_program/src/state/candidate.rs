use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]

pub struct Candidate {
    #[max_len(32)]
    pub candidate_name: String,
    pub candidate_vote: u64,
}
