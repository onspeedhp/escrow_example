use anchor_lang::prelude::*;
mod error;
mod instructions;
pub mod state;

pub use error::*;
use instructions::*;
pub use state::*;

declare_id!("EEVctJWhQ3Ag9u5H8XLYTai5sWsHWsTNJ2YrcJAP8D2g");

#[program]
pub mod escrow_example {
    use super::*;

    pub fn init_escrow(ctx: Context<InitEscrow>, args: InitEscrowArgs) -> Result<()> {
        instructions::init_escrow(ctx, args)
    }

    pub fn confirm_receipt(ctx: Context<ConfirmReceipt>) -> Result<()> {
        instructions::confirm_receipt(ctx)
    }

    pub fn resolve_partial(ctx: Context<ResolvePartial>, received_quantity: u64) -> Result<()> {
        instructions::resolve_partial(ctx, received_quantity)
    }

    pub fn claim_timeout(ctx: Context<ClaimTimeout>) -> Result<()> {
        instructions::claim_timeout(ctx)
    }
}
