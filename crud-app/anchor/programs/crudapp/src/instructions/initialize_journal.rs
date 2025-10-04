use anchor_lang::prelude::*;

use crate::state::Journal;

#[derive(Accounts)]
pub struct IniitalizeJournal<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + Journal::INIT_SPACE,
        seeds = [b"journal", user.key().as_ref()],
        bump
    )]
    pub journal: Account<'info, Journal>,

    pub system_program: Program<'info, System>,
}

impl<'info> IniitalizeJournal<'info> {
    pub fn init_journal(&mut self, title: String, message: String) -> Result<()> {
        self.journal.set_inner(Journal {
            owner: self.user.key(),
            title,
            message,
        });
        Ok(())
    }
}
