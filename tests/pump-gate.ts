import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PumpGate } from "../target/types/pump_gate";
import {
  PublicKey,
  Keypair,
  SystemProgram,
} from "@solana/web3.js";
import {
  getAssociatedTokenAddress,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { assert } from "chai";

// PUMP token mint on devnet (replace with real address)
const PUMP_MINT = new PublicKey("REPLACE_WITH_PUMP_MINT_ADDRESS");

// pump.fun program ID
const PUMP_FUN_PROGRAM_ID = new PublicKey(
  "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"
);

describe("pump-gate", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PumpGate as Program<PumpGate>;
  const user = provider.wallet as anchor.Wallet;

  it("Fails with NOT ENOUGH $PUMP HELD when balance is zero", async () => {
    const userPumpAta = await getAssociatedTokenAddress(
      PUMP_MINT,
      user.publicKey
    );

    try {
      await program.methods
        .gatedLaunch(Buffer.from([]))
        .accounts({
          user: user.publicKey,
          userPumpTokenAccount: userPumpAta,
          pumpMint: PUMP_MINT,
          pumpFunProgram: PUMP_FUN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      assert.fail("Should have thrown NotEnoughPump error");
    } catch (err: any) {
      assert.include(
        err.message,
        "NOT ENOUGH $PUMP HELD",
        "Wrong error message"
      );
      console.log("✅ Correctly rejected: NOT ENOUGH $PUMP HELD");
    }
  });

  it("Passes gate when wallet holds 25,000+ PUMP", async () => {
    // NOTE: For this test to pass you need to actually hold PUMP in your wallet
    // This is just the structure — fund your wallet with PUMP first
    console.log(
      "ℹ️  Fund your wallet with 25,000 PUMP on devnet then run this test"
    );
  });
});
