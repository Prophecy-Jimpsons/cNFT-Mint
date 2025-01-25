use crate::state::metadata::create_metadata;
use crate::utils::mint_bubblegum_cnft;
use anchor_lang::prelude::*;
use mpl_bubblegum::{instructions, ID as BUBBLEGUM_PROGRAM_ID};
use spl_account_compression::program::SplAccountCompression;
use spl_account_compression::Noop;

#[derive(Accounts)]
#[instruction()]
pub struct MintCNFT<'info> {
    /// CHECK: This is the tree authority PDA, verified in CPI to Bubblegum program
    #[account(mut)]
    pub tree_authority: AccountInfo<'info>,

    #[account(mut)]
    pub leaf_owner: Signer<'info>,

    #[account(mut)]
    pub leaf_delegate: Signer<'info>,

    /// CHECK: This is the merkle tree account that stores compressed NFT data
    #[account(mut)]
    pub merkle_tree: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub log_wrapper: Program<'info, Noop>,

    pub compression_program: Program<'info, SplAccountCompression>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint_record: AccountInfo<'info>,

    /// CHECK: Bubblegum Program
    #[account(address = BUBBLEGUM_PROGRAM_ID)]
    pub bubblegum_program: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MetadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub collection_key: Option<Pubkey>,
    pub creators: Vec<Creator>,
    pub edition_nonce: Option<u8>,
    pub uses: Option<Uses>,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
    pub seller_fee_basis_points: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: bool,
    pub share: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Uses {
    pub use_method: UseMethod,
    pub remaining: u64,
    pub total: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum UseMethod {
    Burn,
    Multiple,
    Single,
}

pub fn mint_cnft(
    ctx: Context<MintCNFT>,
    name: String,
    symbol: String,
    uri: String,
    jimp_allocation: u64,
) -> Result<()> {
    // Create metadata for the NFT
    let metadata = create_metadata(&name, &symbol, &uri, jimp_allocation)?;

    // Mint the compressed NFT using Bubblegum program
    mint_bubblegum_cnft(&ctx, metadata)?;

    // Log the allocation and minting details
    msg!("JIMP allocation for NFT: {}", jimp_allocation);
    msg!("Compressed NFT minted successfully");
    msg!("Tree Authority: {}", ctx.accounts.tree_authority.key());
    msg!("Merkle Tree: {}", ctx.accounts.merkle_tree.key());
    msg!("Owner: {}", ctx.accounts.leaf_owner.key());

    Ok(())
}

// Error handling
#[error_code]
pub enum MintError {
    #[msg("Invalid metadata provided")]
    InvalidMetadata,
    #[msg("Invalid allocation amount")]
    InvalidAllocation,
    #[msg("Mint operation failed")]
    MintFailed,
}
