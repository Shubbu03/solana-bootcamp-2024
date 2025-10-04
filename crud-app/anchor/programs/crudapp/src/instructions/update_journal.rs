use anchor_lang::{prelude::*, solana_program::message};

use crate::state::Journal;

#[derive(Accounts)]
pub struct UpdateJournal<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"journal", user.key().as_ref()],
        bump,
        realloc = 8 + Journal::INIT_SPACE, // this is needed as if the size of message is increased or decresed the rent will change , so we recalulate it 
        realloc::payer = user,
        realloc::zero = true, // initial space calculation is turned to zero first then recalculated
    )]
    pub journal: Account<'info, Journal>,

    pub system_program: Program<'info, System>,
}

impl<'info> UpdateJournal<'info> {
    pub fn update_journal(&mut self, message: String) -> Result<()> {
        //validate messge not null/empty

        self.journal.message = message;
        Ok(())
    }
}
