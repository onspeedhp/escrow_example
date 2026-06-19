use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::instructions::confirm_receipt::transfer_from_vault;
use crate::{Escrow, EscrowError, EscrowStatus};

pub fn claim_timeout(ctx: Context<ClaimTimeout>) -> Result<()> {
    let escrow_account = &mut ctx.accounts.escrow_account;

    require!(
        escrow_account.status == EscrowStatus::Funded,
        EscrowError::InvalidStatus
    );
    require!(
        ctx.accounts.seller.key() == escrow_account.seller,
        EscrowError::UnauthorizedSeller
    );
    require!(
        Clock::get()?.unix_timestamp >= escrow_account.delivery_deadline,
        EscrowError::TimeoutNotReached
    );

    transfer_from_vault(
        escrow_account,
        ctx.accounts.vault_authority.to_account_info(),
        ctx.accounts.vault_token_account.to_account_info(),
        ctx.accounts.seller_token_account.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.mint.decimals,
        escrow_account.amount,
    )?;

    escrow_account.released_amount = escrow_account.amount;
    escrow_account.received_quantity = escrow_account.quantity;
    escrow_account.status = EscrowStatus::ClaimedByTimeout;

    Ok(())
}

#[derive(Accounts)]
pub struct ClaimTimeout<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        mut,
        seeds = [
            Escrow::PREFIX_SEED,
            escrow_account.buyer.as_ref(),
            escrow_account.seller.as_ref(),
            &escrow_account.escrow_id.to_le_bytes(),
        ],
        bump = escrow_account.escrow_bump,
    )]
    pub escrow_account: Box<Account<'info, Escrow>>,

    #[account(
        constraint = mint.key() == escrow_account.mint
    )]
    pub mint: Box<Account<'info, Mint>>,

    #[account(
        seeds = [Escrow::VAULT_AUTHORITY_SEED, escrow_account.key().as_ref()],
        bump = escrow_account.vault_authority_bump,
    )]
    /// CHECK: PDA authority for the vault token account.
    pub vault_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [Escrow::VAULT_SEED, escrow_account.key().as_ref()],
        bump = escrow_account.vault_bump,
        constraint = vault_token_account.key() == escrow_account.vault,
        constraint = vault_token_account.mint == escrow_account.mint,
        constraint = vault_token_account.owner == vault_authority.key()
    )]
    pub vault_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = seller_token_account.owner == escrow_account.seller,
        constraint = seller_token_account.mint == escrow_account.mint
    )]
    pub seller_token_account: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
}
