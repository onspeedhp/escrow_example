import { AnchorProvider } from '@coral-xyz/anchor';
import { Keypair, SystemProgram, Transaction } from '@solana/web3.js';

export const createAccount = async ({
  provider,
  newAccountKeypair,
  lamports,
}: {
  provider: AnchorProvider;
  newAccountKeypair: Keypair;
  lamports: number;
}) => {
  const dataLength = 0;

  const rentExemptionAmount =
    await provider.connection.getMinimumBalanceForRentExemption(dataLength);

  const createAccountIns = SystemProgram.createAccount({
    fromPubkey: provider.wallet.publicKey,
    newAccountPubkey: newAccountKeypair.publicKey,
    lamports: rentExemptionAmount,
    space: dataLength,
    programId: SystemProgram.programId,
  });

  const transferIns = SystemProgram.transfer({
    fromPubkey: provider.wallet.publicKey,
    toPubkey: newAccountKeypair.publicKey,
    lamports: lamports,
  });

  const tx = new Transaction().add(createAccountIns).add(transferIns);

  const sig = await provider.sendAndConfirm(tx, [newAccountKeypair], {
    maxRetries: 20,
  });

  console.log(
    `Create account ${newAccountKeypair.publicKey} with ${lamports} lamports: ${sig}`
  );
};

export function u16ToBytes(value: number): Uint8Array {
  const buffer = new ArrayBuffer(2); // u16 cần 2 bytes
  const view = new DataView(buffer);
  view.setUint16(0, value, true); // true để lưu theo Little Endian, false để lưu theo Big Endian
  return new Uint8Array(buffer);
}

export function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
