import * as anchor from "@coral-xyz/anchor";
import {
  getAssociatedTokenAddress,
  getAssociatedTokenAddressSync,
  createMint as splCreateMint,
} from "@solana/spl-token";
import { Keypair, PublicKey, Transaction } from "@solana/web3.js";

export async function createMint(
  provider: anchor.Provider,
  mint: Keypair,
  authority: PublicKey
): Promise<void> {
  let vault = getAssociatedTokenAddressSync(mint, authority, true);

  //create the mint here

  return mint.publicKey;
}
