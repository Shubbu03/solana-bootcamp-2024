use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Favourites {
	pub number: u64,
	#[max_len(32)]
	pub color: String,
	#[max_len(5, 32)] // 5 for vec, 32 for string
	pub hobbies: Vec<String>
}
