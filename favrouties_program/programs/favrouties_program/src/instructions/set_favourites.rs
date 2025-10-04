use anchor_lang::prelude::*;

use crate::Favourites;

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
		init,
		payer = user,
		space = 8 + Favourites::INIT_SPACE,
		seeds = [b"favourites", user.key().as_ref()],
		bump
	)]
    pub favourites: Account<'info, Favourites>,

    pub system_program: Program<'info, System>,
}

impl<'info> SetFavorites<'info> {
    pub fn set_fav(&mut self, number: u64, color: String, hobbies: Vec<String>) -> Result<()> {
        self.favourites.set_inner(Favourites {
            number,
            color,
            hobbies,
        });
        Ok(())
    }
}
