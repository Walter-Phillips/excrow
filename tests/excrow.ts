import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Excrow } from "../target/types/excrow";

describe("excrow", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Excrow as Program<Excrow>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
