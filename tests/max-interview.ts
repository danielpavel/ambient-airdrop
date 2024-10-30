import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { createMint } from "@solana/spl-token";
import { Keypair, PublicKey } from "@solana/web3.js";
import { MaxInterview } from "../target/types/max_interview";

describe("max-interview", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const connection = provider.connection;

  const program = anchor.workspace.MaxInterview as Program<MaxInterview>;

  //let maker: PublicKey = PublicKey.unique();

  let airdropStateAccount: PublicKey;
  let mint: Keypair = Keypair.generate();

  let maker = provider.publicKey;

  before("before setup", async () => {
    //await connection.requestAirdrop(maker, lamports)
    let [airdropStateAccount, _bump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("airdrop"),
        mint.publicKey.toBuffer(),
        maker.toBuffer(),
      ],
      program.programId
    );

    // create mint
    await createMint(provider, mint, airdropStateAccount);
  });

  it("Is initialized!", async () => {
    // Add your test here.
    let accounts = {};
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
