use anchor_lang::prelude::*;

use crate::{errors::ApiKeysErrors, state::*};

#[derive(Accounts)]
#[instruction(key : Vec<u8>)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 12 + ApiKeyAccount::MAX_SIZE,
        seeds = [b"apikey", authority.key().as_ref(), key.as_ref()],
        bump
    )]
    pub apikeys_account: Account<'info, ApiKeyAccount>,

    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>, key: String, limit: u32) -> Result<()> {
    let account = &mut ctx.accounts.apikeys_account;

    require_gte!(
        ApiKeyAccount::KEY_MAX_SIZE,
        key.len(),
        ApiKeysErrors::KeyMaxSizeOverflow
    );

    account.authority = ctx.accounts.authority.key();
    account.key = key;
    account.limit = limit;
    account.bump = ctx.bumps.apikeys_account;

    Ok(())
}
