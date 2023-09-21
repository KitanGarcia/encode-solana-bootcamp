import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Homework13 } from "../target/types/homework13";

import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { IDL } from "../target/types/hw13";

export function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

describe("homework13", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Homework13 as Program<Homework13>;

  it("Is initialized!", async () => {
    const balanceStructKeypair = Keypair.generate();

    try {
      const tx = await program.methods
        .initialize()
        .accounts({
          balanceStruct: balanceStructKeypair.publicKey,
          authority: provider.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([balanceStructKeypair])
        .rpc();

      console.log("Your transaction signature:", tx);

      // await sleep(20000); // not work then take higer value

      const balancesStructAccount = await program.account.balanceStruct.fetch(
        balanceStructKeypair.publicKey
      );

      console.log("===================================================");
      console.log("Balance:", balancesStructAccount.balance.toString());
    } catch (error) {
      console.log("ERROR:", error);
    }
  });
});
