use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct ApiKeyAccount {
    pub key: String,
    pub authority: Pubkey,
    pub limit: u32,
    pub bump: u8,
}

impl ApiKeyAccount {
    pub const KEY_MAX_SIZE: usize = 32;

    /// 0. key - I think it would be UUID hashed with argon2 and represented as hex - so 32
    /// 1. authority - Pubkey is 32 long - https://www.anchor-lang.com/docs/references/space
    /// 2. limit - 4
    /// 3. bump - 1
    pub const MAX_SIZE: usize = Self::KEY_MAX_SIZE + 32 + 4 + 1;
}
