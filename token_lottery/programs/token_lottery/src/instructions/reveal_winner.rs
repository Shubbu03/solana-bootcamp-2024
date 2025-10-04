use anchor_lang::prelude::*;
use switchboard_on_demand::RandomnessAccountData;

use crate::{error::ErrorCode, Lottery};

#[derive(Accounts)]
pub struct RevealWinner<'info> {
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
    // pub system_program: Program<'info, System>,
}

impl<'info> RevealWinner<'info> {
    pub fn reveal_lottery_winner(&mut self) -> Result<()> {
        let clock = Clock::get()?;
        let token_lottery = &mut self.lottery;

        if self.payer.key() != token_lottery.authority {
            return Err(ErrorCode::NotAuthorised.into());
        }

        if self.randomness_account.key() != token_lottery.randomness_account {
            return Err(ErrorCode::IncorrectRandomnessAccount.into());
        }

        if clock.slot < token_lottery.end_time {
            return Err(ErrorCode::LotteryNotCompleted.into());
        }

        require!(!token_lottery.winner_chosen, ErrorCode::WinnerChosen);

        let randonmess_data =
            RandomnessAccountData::parse(self.randomness_account.data.borrow()).unwrap();

        let revealed_random_value = randonmess_data
            .get_value(&clock)
            .map_err(|_| ErrorCode::RandomnessNotResolved)?;

        let winner: u64 = revealed_random_value[0] as u64 % token_lottery.total_tickets;

        token_lottery.winner = winner;
        token_lottery.winner_chosen = true;

        Ok(())
    }
}
