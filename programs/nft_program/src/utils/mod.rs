use crate::error::NFTError;
use crate::instructions::MintCNFT;
use anchor_lang::prelude::*;
use mpl_bubblegum::instructions::{MintV1, MintV1InstructionArgs};
use mpl_bubblegum::types::MetadataArgs;

pub fn mint_bubblegum_cnft<'info>(ctx: &Context<MintCNFT>, metadata: MetadataArgs) -> Result<()> {
    let mint_ix = MintV1 {
        tree_config: ctx.accounts.tree_authority.key(),
        leaf_owner: ctx.accounts.leaf_owner.key(),
        leaf_delegate: ctx.accounts.leaf_delegate.key(),
        merkle_tree: ctx.accounts.merkle_tree.key(),
        payer: ctx.accounts.payer.key(),
        tree_creator_or_delegate: ctx.accounts.payer.key(),
        log_wrapper: ctx.accounts.log_wrapper.key(),
        compression_program: ctx.accounts.compression_program.key(),
        system_program: ctx.accounts.system_program.key(),
    };

    // Create instruction with args
    let ix = mint_ix.instruction(MintV1InstructionArgs { metadata });

    // Execute the instruction
    solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.tree_authority.to_account_info(),
            ctx.accounts.leaf_owner.to_account_info(),
            ctx.accounts.leaf_delegate.to_account_info(),
            ctx.accounts.merkle_tree.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.log_wrapper.to_account_info(),
            ctx.accounts.compression_program.to_account_info(),
        ],
    )
    .map_err(|_| NFTError::MintFailed)?;

    Ok(())
}
