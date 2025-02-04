import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaStore } from "../target/types/solana_store";
import { assert, expect } from "chai";

describe("solana-store", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaStore as Program<SolanaStore>;
  const addr = "c312fa79e86049a12a9924a86283cf0b";

  it("Create PDA account", async () => {
    // Create new PDA account then query it and check limit is set correctly
    await program.methods.initialize(addr, 10).rpc();
    const keys = (await program.account.apiKeyAccount.all())[0];
     
    assert(keys.account.key == addr);
    assert(keys.account.limit == 10);
  });
});