use anchor_lang::error_code;

#[error_code]
pub enum EscrowError {
    #[msg("Time is invalid to withdraw funds")]
    InvalidTimeToWithdraw,

    #[msg("Admin is already withdraw this escrow")]
    AlreadyWithdraw,

    #[msg("Invalid time to claim funds")]
    InvalidTimeToClaim,
}
