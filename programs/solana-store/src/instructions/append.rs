use anchor_lang::prelude::*;

use crate::{errors::ApiKeysErrors, state::DataAccount};

#[derive(Accounts)]
pub struct AppendApiAccount<'info> {
    #[account(mut)]
    pub data_account: Account<'info, DataAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

pub fn append_new_key(
    ctx: Context<AppendApiAccount<'_>>,
    pubkey: Vec<u8>,
    limit: u32,
) -> Result<()> {
    let storage = &mut ctx.accounts.data_account.data;

    require!(storage.contains_key(&pubkey), ApiKeysErrors::KeyExists);

    let _ = storage.insert(pubkey, limit);

    Ok(())
}
