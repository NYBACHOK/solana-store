#[anchor_lang::error_code]
pub enum ApiKeysErrors {
    NotFound,
    KeyExists,
}
