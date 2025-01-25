use anchor_lang::prelude::*;

#[error_code]
pub enum NFTError {
    #[msg("Invalid metadata provided")]
    InvalidMetadata,
    #[msg("Invalid allocation amount")]
    InvalidAllocation,
    #[msg("Invalid merkle tree")]
    InvalidMerkleTree,
    #[msg("Mint failed")]
    MintFailed,
}
