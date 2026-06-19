use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub escrow_id: u64,
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub amount: u64,
    pub quantity: u64,
    pub delivery_deadline: i64,
    pub released_amount: u64,
    pub received_quantity: u64,
    pub status: EscrowStatus,
    pub escrow_bump: u8,
    pub vault_authority_bump: u8,
    pub vault_bump: u8,
}

impl Escrow {
    pub const PREFIX_SEED: &'static [u8] = b"escrow";
    pub const VAULT_AUTHORITY_SEED: &'static [u8] = b"vault_authority";
    pub const VAULT_SEED: &'static [u8] = b"vault";

    pub const LEN: usize =
        8 + // escrow_id
        32 + // buyer
        32 + // seller
        32 + // mint
        32 + // vault
        8 + // amount
        8 + // quantity
        8 + // delivery_deadline
        8 + // released_amount
        8 + // received_quantity
        1 + // status
        1 + // escrow_bump
        1 + // vault_authority_bump
        1; // vault_bump
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum EscrowStatus {
    Funded,
    Released,
    PartiallyResolved,
    ClaimedByTimeout,
}
