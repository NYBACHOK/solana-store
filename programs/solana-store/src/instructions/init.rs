use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(key : Vec<u8>)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = ApiKeyAccount::MAX_SIZE,
        seeds = [b"apikey", authority.key().as_ref(), key.as_ref()],
        bump
    )]
    pub apikeys_account: Account<'info, ApiKeyAccount>,

    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>, key: Vec<u8>, limit: u64) -> Result<()> {
    let account = &mut ctx.accounts.apikeys_account;

    account.authority = ctx.accounts.authority.key();
    account.key = key;
    account.limit = limit;

    Ok(())
}
