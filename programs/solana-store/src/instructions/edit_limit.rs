use anchor_lang::prelude::*;

use crate::{state::*, errors::*};

#[derive(Accounts)]
#[instruction(key : Vec<u8>)]
pub struct EditLimit<'info> {
    #[account()]
    pub authority: Signer<'info>,

    #[account(
        mut, 
        seeds = [b"apikeys", authority.key().as_ref(), key.as_ref()],
        bump
    )]
    pub apikeys_account: Account<'info, ApiKeyAccount>,

    pub system_program: Program<'info, System>,
}


pub fn edit_limit(ctx: Context<EditLimit>, key: Vec<u8>, new_limit: u64) -> Result<()> {
    let account = &mut ctx.accounts.apikeys_account;

    require_keys_eq!(account.authority, ctx.accounts.authority.key(),  ApiKeysErrors::InvalidAuthority);
    require!(account.key == key, ApiKeysErrors::DifferentKeysLoaded);
    
    account.limit = new_limit;

    Ok(())
}

