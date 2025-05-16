import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorZkemail } from "../target/types/anchor_zkemail";

describe("anchor_zkemail", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.anchorZkemail as Program<AnchorZkemail>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  
  it("Handles bonsol_callback with dummy data", async () => {
    const dummyData = Buffer.from("dummy");

    try {
      const tx = await program.methods
        .bonsolCallback(dummyData)
        .accounts({
          requestAccount: provider.wallet.publicKey,
        })
        .rpc();
  
      console.log("✅ bonsol_callback tx:", tx);
    } catch (SendTransactionError) {
      console.error("❌ Error sending transaction:", SendTransactionError);
      SendTransactionError.getLogs();
    }
  });
});

