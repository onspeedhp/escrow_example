use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked};

use crate::{Escrow, EscrowError, EscrowStatus};

pub fn confirm_receipt(ctx: Context<ConfirmReceipt>) -> Result<()> {
    let escrow_account = &mut ctx.accounts.escrow_account;

    require!(
        escrow_account.status == EscrowStatus::Funded,
        EscrowError::InvalidStatus
    );
    require!(
        ctx.accounts.buyer.key() == escrow_account.buyer,
        EscrowError::UnauthorizedBuyer
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
    escrow_account.status = EscrowStatus::Released;

    Ok(())
}

#[derive(Accounts)]
pub struct ConfirmReceipt<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

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

pub(crate) fn transfer_from_vault<'info>(
    escrow_account: &Account<'info, Escrow>,
    vault_authority: AccountInfo<'info>,
    vault_token_account: AccountInfo<'info>,
    destination_token_account: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    decimals: u8,
    amount: u64,
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }

    let escrow_key = escrow_account.key();
    let signer_seeds: &[&[&[u8]]] = &[&[
        Escrow::VAULT_AUTHORITY_SEED,
        escrow_key.as_ref(),
        &[escrow_account.vault_authority_bump],
    ]];

    let transfer_accounts = TransferChecked {
        from: vault_token_account,
        mint,
        to: destination_token_account,
        authority: vault_authority,
    };

    let cpi_ctx = CpiContext::new_with_signer(token_program, transfer_accounts, signer_seeds);

    token::transfer_checked(cpi_ctx, amount, decimals)
}
