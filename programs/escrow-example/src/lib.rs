use anchor_lang::prelude::*;
mod error;
mod instructions;
pub mod state;

pub use error::*;
use instructions::*;
pub use state::*;

declare_id!("EEVctJWhQ3Ag9u5H8XLYTai5sWsHWsTNJ2YrcJAP8D2g");

// 1 day
pub const DEADLINE: i64 = 60 * 60 * 24;

#[program]
pub mod escrow_example {
    use super::*;

    pub fn init_escrow(ctx: Context<InitEscrow>, escrow_args: Escrow) -> Result<()> {
        instructions::init_escrow(ctx, escrow_args)
    }

    pub fn withdraw_funds(
        ctx: Context<WithdrawFunds>,
        escrow_index: u16,
        receiver_index: u8,
    ) -> Result<()> {
        instructions::withdraw_funds(ctx, escrow_index, receiver_index)
    }

    pub fn claim(ctx: Context<Claim>, escrow_index: u16) -> Result<()> {
        instructions::claim(ctx, escrow_index)
    }
}
