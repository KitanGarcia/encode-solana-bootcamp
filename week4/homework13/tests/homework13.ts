import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Homework13 } from "../target/types/homework13";

import { Connection, Keypair, PublicKey } from "@solana/web3.js";

describe("homework13", () => {
  // Configure the client to use the local cluster.
  /*
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.local();
  */
  //anchor.setProvider(anchor.AnchorProvider.local());
  const provider = anchor.AnchorProvider.env();

  const program = anchor.workspace.Homework13 as Program<Homework13>;

  it("Is initialized!", async () => {
    const balanceStructKeypair = Keypair.generate();
    console.log(balanceStructKeypair);
    console.log(provider.wallet.publicKey);

    const tx = await program.methods
      .initialize()
      .accounts({
        balanceStruct: balanceStructKeypair.publicKey,
        authority: provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([balanceStructKeypair])
      .rpc();

    const balancesStructAccount = await program.account.balanceStruct.fetch(
      balanceStructKeypair.publicKey
    );

    console.log("===================================================");
    console.log("Balance:", balancesStructAccount.balance.toString());
    console.log("Your transaction signature:", tx);
  });
});
