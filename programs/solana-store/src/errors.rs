#[anchor_lang::error_code]
pub enum ApiKeysErrors {
    DifferentKeysLoaded = 10,
    InvalidAuthority = 11,
}
