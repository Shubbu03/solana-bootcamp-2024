use anchor_lang::prelude::*;

use crate::{Lottery, DISCRIMINATOR};

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = DISCRIMINATOR + Lottery::INIT_SPACE,
        seeds = [b"lottery".as_ref()],
        bump
    )]
    pub lottery: Account<'info, Lottery>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeConfig<'info> {
    pub fn init_config(
        &mut self,
        start_time: u64,
        end_time: u64,
        ticket_price: u64,
        bump: u8,
    ) -> Result<()> {
        self.lottery.set_inner(Lottery {
            winner: 0,
            winner_chosen: false,
            start_time,
            end_time,
            lottery_pot_amount: 0,
            total_tickets: 0,
            ticket_price,
            authority: *self.payer.key,
            randomness_account: Pubkey::default(),
            bump,
        });

        Ok(())
    }
}
