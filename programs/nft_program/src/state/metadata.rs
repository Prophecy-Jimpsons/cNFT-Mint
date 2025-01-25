use anchor_lang::prelude::*;
use mpl_bubblegum::types::{Creator, MetadataArgs, TokenProgramVersion, TokenStandard};

pub fn create_metadata(
    name: &str,
    symbol: &str,
    uri: &str,
    _jimp_allocation: u64,
) -> Result<MetadataArgs> {
    if name.is_empty() || symbol.is_empty() || uri.is_empty() {
        return Err(error!(crate::error::NFTError::InvalidMetadata));
    }

    // Create default creator
    let creators = vec![Creator {
        address: Pubkey::default(), // Replace with actual creator address
        verified: false,
        share: 100,
    }];

    let metadata = MetadataArgs {
        name: name.to_string(),
        symbol: symbol.to_string(),
        uri: uri.to_string(),
        seller_fee_basis_points: 0,
        primary_sale_happened: false,
        is_mutable: true,
        edition_nonce: None,
        token_standard: Some(TokenStandard::NonFungible), // Set token standard to NonFungible
        collection: None,
        uses: None,
        token_program_version: TokenProgramVersion::Original,
        creators,
    };
    Ok(metadata)
}
