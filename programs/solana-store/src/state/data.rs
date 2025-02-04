use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct ApiKeyAccount {
    pub key: String,
    pub authority: Pubkey,
    pub limit: u32,
}

impl ApiKeyAccount {
    /// 2. key - I think it would be UUID hashed with argon2 and represented as hex - so 32
    /// 3. authority - Pubkey is 32 long - https://www.anchor-lang.com/docs/references/space
    /// 4. limit - 4
    pub const MAX_SIZE: usize = 32 + 32 + 4;
}
