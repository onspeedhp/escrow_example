import * as anchor from "@coral-xyz/anchor";
import { EscrowExample } from "../target/types/escrow_example";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import {
  createMint,
  getAccount,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import { BN } from "bn.js";
import { expect } from "chai";

describe("escrow-example", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .EscrowExample as anchor.Program<EscrowExample>;
  const provider = program.provider as anchor.AnchorProvider;
  const payer = (provider.wallet as any).payer as Keypair;

  it("locks buyer funds and releases them to seller", async () => {
    const buyer = Keypair.generate();
    const seller = Keypair.generate();

    await fund(buyer.publicKey);
    await fund(seller.publicKey);

    const mint = await createMint(
      provider.connection,
      payer,
      payer.publicKey,
      null,
      6
    );

    const buyerTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      mint,
      buyer.publicKey
    );
    const sellerTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      mint,
      seller.publicKey
    );

    const amount = new BN(1_000_000);
    const quantity = new BN(100);
    const escrowId = new BN(Date.now());
    const deliveryDeadline = new BN(Math.floor(Date.now() / 1000) + 60);

    await mintTo(
      provider.connection,
      payer,
      mint,
      buyerTokenAccount.address,
      payer,
      Number(amount)
    );

    const [escrowAccount] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        buyer.publicKey.toBuffer(),
        seller.publicKey.toBuffer(),
        escrowId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );
    const [vaultAuthority] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault_authority"), escrowAccount.toBuffer()],
      program.programId
    );
    const [vaultTokenAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), escrowAccount.toBuffer()],
      program.programId
    );

    await program.methods
      .initEscrow({
        escrowId,
        amount,
        quantity,
        deliveryDeadline,
      })
      .accountsPartial({
        buyer: buyer.publicKey,
        buyerTokenAccount: buyerTokenAccount.address,
        seller: seller.publicKey,
        mint,
        escrowAccount,
        vaultAuthority,
        vaultTokenAccount,
      })
      .signers([buyer])
      .rpc();

    const vaultAfterInit = await getAccount(
      provider.connection,
      vaultTokenAccount
    );
    expect(vaultAfterInit.amount).to.equal(BigInt(amount.toString()));

    await program.methods
      .confirmReceipt()
      .accountsPartial({
        buyer: buyer.publicKey,
        escrowAccount,
        mint,
        vaultAuthority,
        vaultTokenAccount,
        sellerTokenAccount: sellerTokenAccount.address,
      })
      .signers([buyer])
      .rpc();

    const sellerAfterRelease = await getAccount(
      provider.connection,
      sellerTokenAccount.address
    );
    expect(sellerAfterRelease.amount).to.equal(BigInt(amount.toString()));

    const escrowState = await program.account.escrow.fetch(escrowAccount);
    expect(escrowState.releasedAmount.toString()).to.equal(amount.toString());
  });

  async function fund(publicKey: PublicKey) {
    const signature = await provider.connection.requestAirdrop(
      publicKey,
      LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(signature, "confirmed");
  }
});
