use anchor_lang::prelude::*;

use crate::state::Journal;

#[derive(Accounts)]
pub struct DeleteJournal<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"journal", user.key().as_ref()],
        bump,
        close = user
    )]
    pub journal: Account<'info, Journal>,

    pub system_program: Program<'info, System>,
}

impl<'info> DeleteJournal<'info> {
    pub fn delete_journal(&mut self) -> Result<()> {
        self.journal.close(self.user.to_account_info());
        Ok(())
    }
}
