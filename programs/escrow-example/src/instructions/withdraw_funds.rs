use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};

use crate::{program::EscrowExample, Escrow, EscrowError, DEADLINE, ID};

pub fn withdraw_funds(
    ctx: Context<WithdrawFunds>,
    _escrow_index: u16,
    _receiver_index: u8,
) -> Result<()> {
    let escrow_account = &ctx.accounts.escrow_account;

    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    // check if current time is larger than deadline
    if current_timestamp >= escrow_account.start_time + DEADLINE {
        return Err(EscrowError::InvalidTimeToWithdraw.into());
    }

    // transfer funds for receiver
    let transfer_accounts = Transfer {
        from: ctx.accounts.vault_authority.to_account_info().clone(),
        to: ctx
            .accounts
            .receiver_token_account
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
#[instruction(_escrow_index: u16, _receiver_index: u8)]
pub struct WithdrawFunds<'info> {
    #[account(mut)]
    // signer must be contract owner
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [Escrow::PREFIX_SEED, &_escrow_index.to_le_bytes()],
        bump,
        owner = ID,
        close = signer
    )]
    // The escrow account, it will hold all necessary info about the trade.
    pub escrow_account: Account<'info, Escrow>,

    #[account(
        seeds = [b"vault_authority"],
        bump,
    )]
    /// CHECK:
    pub vault_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"token-seed".as_ref(), escrow_account.initializer.as_ref(), escrow_account.mint.as_ref()],
        bump,
        constraint = vault_token_account.owner == vault_authority.key(),
        constraint = vault_token_account.mint == escrow_account.mint,
        close = signer
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = receiver_token_account.owner == escrow_account.receiver[_receiver_index as usize],
        constraint = receiver_token_account.mint == escrow_account.mint
    )]
    pub receiver_token_account: Account<'info, TokenAccount>,

    #[account(constraint = program.programdata_address()? == Some(program_data.key()))]
    pub program: Program<'info, EscrowExample>,

    #[account(constraint = program_data.upgrade_authority_address == Some(signer.key()))]
    pub program_data: Account<'info, ProgramData>,

    pub token_program: Program<'info, Token>,
}
