use anchor_lang::prelude::*;

use crate::Poll;

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Poll::INIT_SPACE,
        seeds = [b"poll", poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializePoll<'info> {
    pub fn init_poll(
        &mut self,
        poll_id: u64,
        description: String,
        poll_start: u64,
        poll_end: u64,
    ) -> Result<()> {
        self.poll.set_inner({
            Poll {
                poll_id,
                description,
                poll_start,
                poll_end,
                candidate_amount: 0,
            }
        });

        Ok(())
    }
}
