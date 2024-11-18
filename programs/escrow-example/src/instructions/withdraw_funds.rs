use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use spl_token::{instruction::transfer_checked, solana_program::program::invoke_signed};

use crate::{program::EscrowExample, Escrow, EscrowError, DEADLINE, ID};

pub fn withdraw_funds(ctx: Context<WithdrawFunds>, _receiver_index: u8) -> Result<()> {
    let escrow_account = &ctx.accounts.escrow_account;
    let token_program = &ctx.accounts.token_program.to_account_info();
    let vault_token_account = &ctx.accounts.vault_token_account.to_account_info();
    let mint = &ctx.accounts.mint;
    let receiver_token_account = &ctx.accounts.receiver_token_account.to_account_info();
    let vault_authority = &ctx.accounts.vault_authority.to_account_info();

    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    // check if current time is larger than deadline
    if current_timestamp >= escrow_account.start_time + DEADLINE {
        return Err(EscrowError::InvalidTimeToWithdraw.into());
    }

    let seeds: &[&[u8]] = &[b"vault_authority"];

    let (_, bump) = Pubkey::find_program_address(&seeds, &ID);

    let seeds_signer = &mut seeds.to_vec();
    let binding = [bump];
    seeds_signer.push(&binding);

    invoke_signed(
        &transfer_checked(
            token_program.key,
            &vault_token_account.key(),
            &mint.key(),
            receiver_token_account.key,
            vault_authority.key,
            &[],
            escrow_account.amount,
            mint.decimals,
        )
        .unwrap(),
        &[
            token_program.clone(),
            vault_token_account.clone(),
            mint.to_account_info().clone(),
            receiver_token_account.clone(),
            vault_authority.clone(),
        ],
        &[seeds_signer],
    )?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(_receiver_index: u8)]
pub struct WithdrawFunds<'info> {
    #[account(mut)]
    // signer must be contract owner
    pub signer: Signer<'info>,

    #[account(
        constraint = escrow_account.mint == mint.key()
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        seeds = [Escrow::PREFIX_SEED, escrow_account.initializer.as_ref(), escrow_account.mint.as_ref()],
        bump,
        owner = ID,
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
