use anchor_lang::prelude::*;

use crate::errors::ApiKeysErrors;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

anchor_lang::declare_id!("7QUvVrjgkCbdrKk9ATwwBTtSgbYGNotLk3vZxAv53bn1");

#[program]
pub mod solana_store {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn append_new_key(
        ctx: Context<AppendApiAccount<'_>>,
        pubkey: Vec<u8>,
        limit: u32,
    ) -> Result<()> {
        instructions::append_new_key(ctx, pubkey, limit)
    }

    pub fn read_key(ctx: Context<ReadAccount<'_>>, pubkey: Vec<u8>) -> Result<u32> {
        let key = instructions::read_key(ctx, &pubkey);
        require!(key.is_none(), ApiKeysErrors::NotFound);

        Ok(key.expect("We know it `Some`"))
    }
}
