use anchor_lang::prelude::*;

declare_id!("B67duxe4Db484Z9kaVc65ydsQm3rWrpbS7xfgQHE9ZY");

// Add these program declarations
pub mod bubblegum {
    use anchor_lang::declare_id;
    declare_id!("BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY");
}

pub mod compression {
    use anchor_lang::declare_id;
    declare_id!("cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK");
}

pub mod noop {
    use anchor_lang::declare_id;
    declare_id!("noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV");
}

pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;

#[program]
pub mod nft_program {
    use super::*;

    pub fn mint_cnft(
        ctx: Context<MintCNFT>,
        name: String,
        symbol: String,
        uri: String,
        jimp_allocation: u64,
    ) -> Result<()> {
        instructions::mint::mint_cnft(ctx, name, symbol, uri, jimp_allocation)
    }
}
