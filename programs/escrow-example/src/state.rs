use anchor_lang::prelude::*;

#[account]
#[derive(Debug, InitSpace)]
pub struct Escrow {
    pub initializer: Pubkey,
    #[max_len(20)]
    pub receiver: Vec<Pubkey>,
    pub mint: Pubkey,
    pub amount: u64,
    pub start_time: i64,
}

impl Escrow {
    pub const PREFIX_SEED: &'static [u8] = b"escrow_account";
}
