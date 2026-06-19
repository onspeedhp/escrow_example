use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked};

use crate::{Escrow, EscrowError, EscrowStatus};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InitEscrowArgs {
    pub escrow_id: u64,
    pub amount: u64,
    pub quantity: u64,
    pub delivery_deadline: i64,
}

pub fn init_escrow(ctx: Context<InitEscrow>, args: InitEscrowArgs) -> Result<()> {
    require!(args.amount > 0, EscrowError::InvalidAmount);
    require!(args.quantity > 0, EscrowError::InvalidQuantity);
    require!(
        args.delivery_deadline > Clock::get()?.unix_timestamp,
        EscrowError::InvalidDeadline
    );

    let buyer = &ctx.accounts.buyer;
    let escrow_account = &mut ctx.accounts.escrow_account;

    escrow_account.escrow_id = args.escrow_id;
    escrow_account.buyer = buyer.key();
    escrow_account.seller = ctx.accounts.seller.key();
    escrow_account.mint = ctx.accounts.mint.key();
    escrow_account.vault = ctx.accounts.vault_token_account.key();
    escrow_account.amount = args.amount;
    escrow_account.quantity = args.quantity;
    escrow_account.delivery_deadline = args.delivery_deadline;
    escrow_account.released_amount = 0;
    escrow_account.received_quantity = 0;
    escrow_account.status = EscrowStatus::Funded;
    escrow_account.escrow_bump = ctx.bumps.escrow_account;
    escrow_account.vault_authority_bump = ctx.bumps.vault_authority;
    escrow_account.vault_bump = ctx.bumps.vault_token_account;

    let transfer_accounts = TransferChecked {
        from: ctx.accounts.buyer_token_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.vault_token_account.to_account_info(),
        authority: buyer.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
    );

    token::transfer_checked(cpi_ctx, args.amount, ctx.accounts.mint.decimals)
}

#[derive(Accounts)]
#[instruction(args: InitEscrowArgs)]
pub struct InitEscrow<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        constraint = buyer_token_account.mint == mint.key(),
        constraint = buyer_token_account.owner == buyer.key()
    )]
    pub buyer_token_account: Box<Account<'info, TokenAccount>>,

    /// CHECK: seller does not need to sign when the buyer creates the escrow template.
    pub seller: UncheckedAccount<'info>,

    pub mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = buyer,
        space = 8 + Escrow::LEN,
        seeds = [
            Escrow::PREFIX_SEED,
            buyer.key().as_ref(),
            seller.key().as_ref(),
            &args.escrow_id.to_le_bytes(),
        ],
        bump
    )]
    pub escrow_account: Box<Account<'info, Escrow>>,

    #[account(
        seeds = [Escrow::VAULT_AUTHORITY_SEED, escrow_account.key().as_ref()],
        bump,
    )]
    /// CHECK: PDA authority for the vault token account.
    pub vault_authority: UncheckedAccount<'info>,

    #[account(
        init,
        seeds = [Escrow::VAULT_SEED, escrow_account.key().as_ref()],
        bump,
        payer = buyer,
        token::mint = mint,
        token::authority = vault_authority,
    )]
    pub vault_token_account: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
