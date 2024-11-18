use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};

use crate::Escrow;

pub fn init_escrow(ctx: Context<InitEscrow>, escrow_args: Escrow) -> Result<()> {
    let initializer = &ctx.accounts.initializer;
    let escrow_account = &mut ctx.accounts.escrow_account;
    let vault_token_account = &ctx.accounts.vault_token_account;
    let initializer_deposit_token_account = &ctx
        .accounts
        .initializer_deposit_token_account
        .to_account_info();

    let token_program = &ctx.accounts.token_program.to_account_info();

    // assign data for escrow
    escrow_account.initializer = initializer.key();
    escrow_account.receiver = escrow_args.receiver;
    escrow_account.mint = ctx.accounts.mint.key();
    escrow_account.amount = escrow_args.amount;
    escrow_account.start_time = escrow_args.start_time;

    // transfer token
    let transfer_accounts = Transfer {
        from: initializer_deposit_token_account.clone(),
        to: vault_token_account.to_account_info().clone(),
        authority: initializer.to_account_info().clone(),
    };

    let transfer_ctx = CpiContext::new(token_program.to_account_info().clone(), transfer_accounts);

    transfer(transfer_ctx, escrow_account.amount)
}

#[derive(Accounts)]
pub struct InitEscrow<'info> {
    #[account(mut)]
    // The account of the person initializing the escrow
    pub initializer: Signer<'info>,

    #[account(
        mut, 
        constraint = initializer_deposit_token_account.mint == mint.key(),
        constraint = initializer_deposit_token_account.owner == initializer.key()
    )]
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,

    /// CHECK:
    pub mint: UncheckedAccount<'info>,

    #[account(
        seeds = [b"vault_authority"],
        bump,
    )]
    /// CHECK:
    pub vault_authority: AccountInfo<'info>,

    #[account(
        init_if_needed,
        seeds = [b"token-seed".as_ref(), initializer.key.as_ref(), mint.key().as_ref()],
        bump,
        payer = initializer,
        token::mint = mint,
        token::authority = vault_authority,
    )]
    pub vault_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = initializer,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [Escrow::PREFIX_SEED, initializer.key.as_ref(), mint.key.as_ref()],
        bump
    )]
    // The escrow account, it will hold all necessary info about the trade.
    pub escrow_account: Box<Account<'info, Escrow>>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
}
