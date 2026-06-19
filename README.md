# Escrow Starter

Anchor escrow starter that teams can clone and adapt for a buyer-seller flow.

This repo includes a basic starter shape:

- `init_escrow`: buyer locks tokens into a vault.
- `confirm_receipt`: buyer confirms receipt, seller receives 100%.
- `resolve_partial`: buyer enters the received quantity, seller receives a partial payout, buyer receives the refund.
- `claim_timeout`: seller claims after the deadline if buyer does not respond.

## Tech Stack

- Solana + Anchor `0.30.1`
- Rust smart contract
- TypeScript tests
- SPL Token

## Setup

```bash
git clone https://github.com/onspeedhp/escrow_example.git
cd escrow_example
yarn install
```

Requirements:

- Rust
- Solana CLI
- Anchor CLI `0.30.1`
- Node.js/Yarn

If using `avm`:

```bash
avm use 0.30.1
```

## Env

Create `.env` from the example file:

```bash
cp .env.example .env
```

`PRIVATE_KEY` is the base58 secret key for a dev wallet used by tests to mint tokens and fund accounts.

## Quick Start

Build program:

```bash
yarn build
```

Run local tests:

```bash
yarn test
```

Format code:

```bash
yarn lint:fix
```

## Files To Read First

- `programs/escrow-example/src/state.rs`: escrow account state.
- `programs/escrow-example/src/instructions/init_escrow.rs`: creates the escrow and locks tokens into the vault.
- `programs/escrow-example/src/instructions/confirm_receipt.rs`: releases 100% to the seller.
- `programs/escrow-example/src/instructions/resolve_partial.rs`: releases a partial payout and refund.
- `programs/escrow-example/src/instructions/claim_timeout.rs`: lets the seller claim after timeout.
- `tests/escrow-example.ts`: TypeScript example for calling the program.

## Current Flow

```text
buyer -> init_escrow -> tokens go into the PDA vault
buyer -> confirm_receipt -> seller receives 100%
buyer -> resolve_partial(qty) -> seller receives by qty, buyer receives refund
seller -> claim_timeout -> seller receives 100% after deadline
```

## Notes

- For hackathons, start on localnet/devnet first.
- If real USDC is too much setup, use a test SPL token like the current test does.
- This is a starter, not a production escrow.
- For stricter settlement, require the seller to co-sign `resolve_partial`.
