use anchor_lang::prelude::*;

use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

anchor_lang::declare_id!("7QUvVrjgkCbdrKk9ATwwBTtSgbYGNotLk3vZxAv53bn1");

#[program]
pub mod solana_store {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, key: String, limit: u32) -> Result<()> {
        instructions::initialize(ctx, key, limit)
    }
    pub fn edit_limit(ctx: Context<EditLimit>, key: String, new_limit: u32) -> Result<()> {
        instructions::edit_limit(ctx, key, new_limit)
    }
}
