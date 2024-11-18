import * as anchor from '@coral-xyz/anchor';
import { EscrowExample } from '../target/types/escrow_example';
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  Transaction,
} from '@solana/web3.js';
import { createAccount, u16ToBytes } from './utils';
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from '@solana/spl-token';
import * as bs58 from 'bs58';
import * as dotenv from 'dotenv';
import { BN } from 'bn.js';
import { expect } from 'chai';

dotenv.config();

describe('escrow-example', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .EscrowExample as anchor.Program<EscrowExample>;
  const anchorProvider = program.provider as anchor.AnchorProvider;
  const keypair = Keypair.fromSecretKey(bs58.decode(process.env.PRIVATE_KEY));

  it('Init escrow', async () => {
    // prepare new account
    const newAccount = Keypair.generate();

    // create a new account
    await createAccount({
      provider: anchorProvider,
      newAccountKeypair: newAccount,
      lamports: LAMPORTS_PER_SOL,
    });

    // create a new token
    const mint = await createMint(
      anchorProvider.connection,
      keypair,
      anchorProvider.wallet.publicKey,
      null,
      9
    );

    // create destination token account
    const destination = await getOrCreateAssociatedTokenAccount(
      anchorProvider.connection,
      keypair,
      mint,
      newAccount.publicKey
    );

    // mint for new user
    const sig = await mintTo(
      anchorProvider.connection,
      keypair,
      mint,
      destination.address,
      keypair,
      1000 * 10 ** 9
    );

    console.log('Mint for user: ', sig);

    // init escrow
    const listReceiver: PublicKey[] = [];
    for (let i = 0; i < 10; i++) {
      listReceiver.push(Keypair.generate().publicKey);
    }

    const [vaultAuthority] = PublicKey.findProgramAddressSync(
      [Buffer.from('')],
      program.programId
    );

    const initEscrowIns = await program.methods
      .initEscrow({
        initializer: newAccount.publicKey,
        receiver: listReceiver,
        mint: mint,
        startTime: new BN(Math.floor(Date.now() / 1000)),
        amount: new BN(1000 * 10 ** 9),
      })
      .accounts({
        initializer: newAccount.publicKey,
        initializerDepositTokenAccount: destination.address,
        mint: mint,
      })
      .instruction();

    const tx = new Transaction().add(initEscrowIns);
    tx.feePayer = newAccount.publicKey;

    const signature = await anchorProvider.connection.sendTransaction(
      tx,
      [newAccount],
      { skipPreflight: true }
    );

    console.log('Init escrow: ', signature);
  });
});
