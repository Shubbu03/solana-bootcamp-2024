#![allow(unexpected_cfgs, deprecated)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FsY3i8PCW3gkqbdEGTQ6TWNr3qNo9QerRCmw9f63pveF");

#[program]
pub mod swap {
    use super::*;

    pub fn make_offer(
        ctx: Context<MakeOffer>,
        token_a_offered_amount: u64,
        id: u64,
        token_b_wanted_amount: u64,
    ) -> Result<()> {
        make_offer::send_offered_amount_to_vault(&ctx, token_a_offered_amount)?;
        make_offer::save_offer(ctx, id, token_b_wanted_amount)?;
        Ok(())
    }

    pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
        take_offer::send_wanted_tokens_to_maker(&ctx)?;
        take_offer::withdraw_and_close_vault(ctx)?;
        Ok(())
    }
}
