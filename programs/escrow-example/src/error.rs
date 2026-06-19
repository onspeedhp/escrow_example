use anchor_lang::error_code;

#[error_code]
pub enum EscrowError {
    #[msg("Amount must be greater than zero")]
    InvalidAmount,

    #[msg("Quantity must be greater than zero")]
    InvalidQuantity,

    #[msg("Delivery deadline must be in the future")]
    InvalidDeadline,

    #[msg("Escrow is not in the expected status")]
    InvalidStatus,

    #[msg("Received quantity cannot exceed agreed quantity")]
    ReceivedQuantityTooHigh,

    #[msg("Only the buyer can call this instruction")]
    UnauthorizedBuyer,

    #[msg("Only the seller can call this instruction")]
    UnauthorizedSeller,

    #[msg("Timeout has not passed yet")]
    TimeoutNotReached,

    #[msg("Math overflow")]
    MathOverflow,
}
