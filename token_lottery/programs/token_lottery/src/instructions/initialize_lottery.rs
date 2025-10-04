use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3,
        mpl_token_metadata::types::{CollectionDetails, Creator, DataV2},
        sign_metadata, CreateMasterEditionV3, CreateMetadataAccountsV3, Metadata, SignMetadata,
    },
    token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface},
};

use crate::{NAME, SYMBOL, URI};

#[derive(Accounts)]
pub struct InitializeLottery<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = collection_mint,
        mint::freeze_authority = collection_mint,
        seeds = [b"collection_mint".as_ref()],
        bump
    )]
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = payer,
        token::mint = collection_mint,
        token::authority = collection_token_account,
        seeds = [b"collection_associated_token".as_ref()],
        bump
    )]
    pub collection_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), collection_mint.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    /// CHECK: account checked by metadata program
    pub metadata: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), collection_mint.key().as_ref(), b"edition"],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    /// CHECK: account checked by metadata program
    pub master_edition: UncheckedAccount<'info>,

    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> InitializeLottery<'info> {
    pub fn init_lottery(&mut self, collection_mint_bumps: u8) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[b"collection_mint".as_ref(), &[collection_mint_bumps]]];

        // creating mint account
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = MintTo {
            mint: self.collection_mint.to_account_info(),
            to: self.collection_token_account.to_account_info(),
            authority: self.collection_mint.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_ctx, 1)?;

        // create metadata account
        let metadata_cpi_program = self.token_metadata_program.to_account_info();

        let metadata_cpi_accounts = CreateMetadataAccountsV3 {
            metadata: self.metadata.to_account_info(),
            mint: self.collection_mint.to_account_info(),
            mint_authority: self.collection_mint.to_account_info(),
            payer: self.payer.to_account_info(),
            update_authority: self.collection_mint.to_account_info(),
            system_program: self.system_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };

        let metadata_cpi_ctx =
            CpiContext::new_with_signer(metadata_cpi_program, metadata_cpi_accounts, &signer_seeds);

        let create_metadata_data = DataV2 {
            name: NAME.to_string(),
            symbol: SYMBOL.to_string(),
            uri: URI.to_string(),
            seller_fee_basis_points: 0,
            creators: Some(vec![Creator {
                address: self.collection_mint.key(),
                verified: false,
                share: 100,
            }]),
            collection: None,
            uses: None,
        };

        let collection_details = Some(CollectionDetails::V1 { size: 0 });

        create_metadata_accounts_v3(
            metadata_cpi_ctx,
            create_metadata_data,
            true,
            true,
            collection_details,
        )?;

        // create master edition
        let master_editon_cpi_program = self.token_metadata_program.to_account_info();

        let master_editon_cpi_accounts = CreateMasterEditionV3 {
            edition: self.master_edition.to_account_info(),
            mint: self.collection_mint.to_account_info(),
            update_authority: self.collection_mint.to_account_info(),
            mint_authority: self.collection_mint.to_account_info(),
            payer: self.payer.to_account_info(),
            metadata: self.metadata.to_account_info(),
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

        let verify_collection_cpi_accounts = SignMetadata {
            creator: self.collection_mint.to_account_info(),
            metadata: self.metadata.to_account_info(),
        };

        let verify_collection_ctx = CpiContext::new_with_signer(
            verify_collection_cpi_program,
            verify_collection_cpi_accounts,
            signer_seeds,
        );

        sign_metadata(verify_collection_ctx)?;

        Ok(())
    }
}
