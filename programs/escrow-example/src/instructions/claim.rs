use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};

use crate::{Escrow, EscrowError, DEADLINE, ID};

pub fn claim(ctx: Context<Claim>) -> Result<()> {
    let escrow_account = &ctx.accounts.escrow_account;


    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    // check if current time is larger than deadline
    if current_timestamp < escrow_account.start_time + DEADLINE {
        return Err(EscrowError::InvalidTimeToWithdraw.into());
    }

    // transfer funds for in
    let transfer_accounts = Transfer {
        from: ctx.accounts.vault_authority.to_account_info().clone(),
        to: ctx
            .accounts
            .initializer_deposit_token_account
            .to_account_info()
            .clone(),
        authority: ctx.accounts.vault_authority.to_account_info().clone(),
    };

    let seeds: &[&[u8]] = &[b"vault_authority"];

    let (_, bump) = Pubkey::find_program_address(&seeds, &ID);

    let seeds_signer = &mut seeds.to_vec();
    let binding = [bump];
    seeds_signer.push(&binding);

    let bind: &[&[&[u8]]] = &[seeds_signer];

    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info().clone(),
        transfer_accounts,
        bind,
    );

    transfer(transfer_ctx, escrow_account.amount)
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(
        mut,
        // check if signer must be initializer
        constraint = escrow_account.initializer == signer.key()
    )]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [Escrow::PREFIX_SEED, escrow_account.initializer.as_ref(), escrow_account.mint.as_ref()],
        bump,
        owner = ID,
        close = signer
    )]
    // The escrow account, it hold all necessary info about the trade.
    pub escrow_account: Account<'info, Escrow>,

     #[account(
        mut, 
        constraint = initializer_deposit_token_account.mint == escrow_account.mint,
        constraint = initializer_deposit_token_account.owner == signer.key()
    )]
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"vault_authority"],
        bump,
    )]
    /// CHECK:
    pub vault_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = vault_token_account.owner == vault_authority.key(),
        constraint = vault_token_account.mint == escrow_account.mint
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>
}
