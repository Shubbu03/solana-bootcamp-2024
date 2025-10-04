#![allow(unexpected_cfgs, deprecated)]
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

declare_id!("FuGL9vptZMQ1Tnd2YBHRzAw4Za2CnjiGQgRyYGzquDjq");

#[program]
pub mod favrouties_program {
    use super::*;

    pub fn set_favourites(
        ctx: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        ctx.accounts.set_fav(number, color, hobbies)
    }
}
