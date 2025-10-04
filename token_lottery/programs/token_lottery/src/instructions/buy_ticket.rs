use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{
    associated_token::AssociatedToken, 
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, 
        mpl_token_metadata::types::DataV2, 
        set_and_verify_sized_collection_item, 
        CreateMasterEditionV3, CreateMetadataAccountsV3, Metadata, SetAndVerifySizedCollectionItem
    }, 
    token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface}
};

use crate::{error::ErrorCode, Lottery, NAME, SYMBOL, URI};

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"lottery".as_ref()],
        bump = lottery.bump
    )]
    pub lottery: Account<'info, Lottery>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = collection_mint,
        mint::freeze_authority = collection_mint,
        mint::token_program = token_program,
        seeds = [lottery.total_tickets.to_le_bytes().as_ref()],
        bump    
    )]
    pub ticket_mint: InterfaceAccount<'info,Mint>,

    #[account(
        mut,
        seeds = [b"collection_mint".as_ref()],
        bump
    )]
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), ticket_mint.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    /// CHECK: account checked by metadata program
    pub ticket_metadata: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), ticket_mint.key().as_ref(), b"edition"],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    /// CHECK: account checked by metadata program
    pub ticket_master_edition: UncheckedAccount<'info>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = ticket_mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program
    )]
    pub destination: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), collection_mint.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    /// CHECK: account checked by metadata program
    pub collection_metadata: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), collection_mint.key().as_ref(), b"edition"],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    /// CHECK: account checked by metadata program
    pub collection_master_edition: UncheckedAccount<'info>,

    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> BuyTicket<'info> {
    pub fn buy_lottery_tickets(&mut self, collection_mint_bumps:u8) -> Result<()> {
        let clock = Clock::get()?;

        let ticket_name = NAME.to_owned() + self.lottery.total_tickets.to_string().as_str();

        if clock.slot < self.lottery.start_time || clock.slot > self.lottery.end_time {
            return Err(ErrorCode::LotteryNotOpen.into());
        }

        //deduct money from buyer and add to lottery pot 
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer{
            from: self.payer.to_account_info(),
            to: self.lottery.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
       
        transfer(cpi_ctx, self.lottery.ticket_price)?;

        //mint ticket to user
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"collection_mint".as_ref(),
            &[collection_mint_bumps]
        ]];

        let mint_cpi_program = self.token_program.to_account_info();

        let mint_cpi_accounts = MintTo{
            mint: self.ticket_mint.to_account_info(),
            to: self.destination.to_account_info(),
            authority: self.collection_mint.to_account_info()
        };

        let mint_cpi_ctx = CpiContext::new_with_signer(mint_cpi_program, mint_cpi_accounts, &signer_seeds);

        mint_to(mint_cpi_ctx, 1)?;

        // create metadata account
        let metadata_cpi_program = self.token_metadata_program.to_account_info();

        let metadata_cpi_accounts = CreateMetadataAccountsV3 {
            metadata: self.ticket_metadata.to_account_info(),
            mint: self.ticket_mint.to_account_info(),
            mint_authority: self.collection_mint.to_account_info(),
            payer: self.payer.to_account_info(),
            update_authority: self.collection_mint.to_account_info(),
            system_program: self.system_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };

        let metadata_cpi_ctx =
            CpiContext::new_with_signer(metadata_cpi_program, metadata_cpi_accounts, &signer_seeds);

        let create_metadata_data = DataV2 {
            name: ticket_name,
            symbol: SYMBOL.to_string(),
            uri: URI.to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        create_metadata_accounts_v3(
            metadata_cpi_ctx,
            create_metadata_data,
            true,
            true,
            None,
        )?;

        // create master edition
        let master_editon_cpi_program = self.token_metadata_program.to_account_info();

        let master_editon_cpi_accounts = CreateMasterEditionV3 {
            edition: self.ticket_master_edition.to_account_info(),
            mint: self.ticket_mint.to_account_info(),
            update_authority: self.collection_mint.to_account_info(),
            mint_authority: self.collection_mint.to_account_info(),
            payer: self.payer.to_account_info(),
            metadata: self.ticket_metadata.to_account_info(),
            token_program: self.token_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };

        let master_editon_cpi_ctx = CpiContext::new_with_signer(
            master_editon_cpi_program,
            master_editon_cpi_accounts,
            &signer_seeds,
        );

        create_master_edition_v3(master_editon_cpi_ctx, Some(0))?;

        //verify collection
        let verify_collection_cpi_program = self.token_metadata_program.to_account_info();

        let verify_collcetion_cpi_accounts = SetAndVerifySizedCollectionItem {
            metadata: self.ticket_metadata.to_account_info(),
            collection_authority: self.collection_mint.to_account_info(),
            payer:self.payer.to_account_info(),
            update_authority:self.collection_mint.to_account_info(),
            collection_mint:self.collection_mint.to_account_info(),
            collection_metadata: self.collection_metadata.to_account_info(),
            collection_master_edition:self.collection_master_edition.to_account_info(),
        };

        let verify_collection_cpi_ctx = CpiContext::new_with_signer(verify_collection_cpi_program, verify_collcetion_cpi_accounts, &signer_seeds);

        set_and_verify_sized_collection_item(verify_collection_cpi_ctx, None)?;

        self.lottery.total_tickets += 1;

        Ok(())
    }
}
