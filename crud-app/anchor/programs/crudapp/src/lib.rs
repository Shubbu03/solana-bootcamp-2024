#![allow(clippy::result_large_err)]

pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;
use state::*;

declare_id!("JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H");

#[program]
pub mod crudapp {
    use super::*;

    pub fn iniitialize_journal(
        ctx: Context<IniitalizeJournal>,
        title: String,
        message: String,
    ) -> Result<()> {
        ctx.accounts.init_journal(title, message)
    }

    pub fn update_journal(ctx: Context<UpdateJournal>, message: String) -> Result<()> {
        ctx.accounts.update_journal(message)
    }

    pub fn delete_journal(ctx: Context<DeleteJournal>) -> Result<()> {
        ctx.accounts.delete_journal()
    }
}
