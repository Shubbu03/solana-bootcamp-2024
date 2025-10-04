use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken, 
    metadata::{Metadata, MetadataAccount}, 
    token_interface::{Mint, TokenAccount, TokenInterface}
};

use crate::{error::ErrorCode, Lottery, NAME};

#[derive(Accounts)]
pub struct ClaimWinnings<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
       mut,
       seeds = [b"lottery".as_ref()],
       bump = lottery.bump
    )]
    pub lottery: Account<'info, Lottery>,

    #[account(
        seeds = [lottery.winner.to_le_bytes().as_ref()],
        bump    
    )]
    pub ticket_mint: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [b"collection_mint".as_ref()],
        bump
    )]
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [b"metadata", token_metadata_program.key().as_ref(), ticket_mint.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    pub ticket_metadata: Account<'info, MetadataAccount>,

    #[account(
        associated_token::mint = ticket_mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program,
    )]
    pub ticker_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"metadata", token_metadata_program.key().as_ref(), collection_mint.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    /// CHECK: account checked by metadata program
    pub collection_metadata: Account<'info, MetadataAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimWinnings<'info> {
    pub fn claim_lottery_win(&mut self) -> Result<()> {
        require!(self.lottery.winner_chosen, ErrorCode::WinnerNotChosen);
        
        require!(self.ticket_metadata.collection.as_ref().unwrap().verified, ErrorCode::CollectionNotVerified);
        
        require!(self.ticket_metadata.collection.as_ref().unwrap().key == self.collection_mint.key(), ErrorCode::IncorrectCollection);

        let ticket_name = NAME.to_owned() + &self.lottery.winner.to_string();

        let metadata_name = self.ticket_metadata.name.replace("\u{0}", "");

        require!(metadata_name == ticket_name, ErrorCode::IncorrectTicket);

        require!(self.ticker_account.amount.gt(&0), ErrorCode::NoTicket);

        **self.lottery.to_account_info().lamports.borrow_mut() -= self.lottery.lottery_pot_amount;
        **self.payer.to_account_info().lamports.borrow_mut() += self.lottery.lottery_pot_amount;

        self.lottery.lottery_pot_amount = 0;

        Ok(())
    }
}
