use anchor_lang::prelude::*;

use crate::state::DataAccount;

#[derive(Accounts)]
pub struct ReadAccount<'info> {
    #[account()]
    pub data_account: Account<'info, DataAccount>,
    #[account()]
    pub user: Signer<'info>,
}

pub fn read_key(ctx: Context<ReadAccount<'_>>, pubkey: &[u8]) -> Option<u32> {
    ctx.accounts.data_account.data.get(pubkey).cloned()
}
