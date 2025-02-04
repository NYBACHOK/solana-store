use anchor_lang::prelude::*;

use crate::state::DataAccount;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 1024)]
    pub data_account: Account<'info, DataAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
    Ok(())
}
