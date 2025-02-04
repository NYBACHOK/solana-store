use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct ApiKeyAccount {
    pub key: Vec<u8>,
    pub authority: Pubkey,
    pub limit: u64,
}

impl ApiKeyAccount {
    /// 1. Prefix `b"apikey"`` 6 chars * 1 byte
    /// 2. key - I think it would be UUID hashed with argon2 and represented as hex - so 32
    /// 3. authority - Pubkey is 32 long - https://www.anchor-lang.com/docs/references/space
    /// 4. limit - 8
    pub const MAX_SIZE: usize = 6 + 32 + 32 + 8;
}
