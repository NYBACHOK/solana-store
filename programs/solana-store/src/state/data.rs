use anchor_lang::prelude::*;
use std::collections::HashMap;

#[anchor_lang::account]
#[derive(Default)]
pub struct DataAccount {
    pub data: HashMap<Vec<u8>, u32>,
}
