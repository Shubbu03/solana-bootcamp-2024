use anchor_lang::prelude::*;
use switchboard_on_demand::RandomnessAccountData;

use crate::{error::ErrorCode, Lottery};

#[derive(Accounts)]
pub struct CommitRandomness<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
       mut,
       seeds = [b"lottery".as_ref()],
       bump = lottery.bump
    )]
    pub lottery: Account<'info, Lottery>,

    ///CHECK: this account is checked by the switchboard program
    pub randomness_account: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> CommitRandomness<'info> {
    pub fn commit_randomness(&mut self) -> Result<()> {
        let clock = Clock::get()?;
        let token_lottery = &mut self.lottery;

        if self.payer.key() != token_lottery.authority {
            return Err(ErrorCode::NotAuthorised.into());
        }

        let randonmess_data =
            RandomnessAccountData::parse(self.randomness_account.data.borrow()).unwrap();

        if randonmess_data.seed_slot != clock.slot - 1 {
            return Err(ErrorCode::RandomnessAlreadyRevealed.into());
        }

        token_lottery.randomness_account = self.randomness_account.key();

        Ok(())
    }
}
