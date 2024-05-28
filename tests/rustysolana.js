import * as anchor from "@project-serum/anchor";
import { web3, setProvider } from "@project-serum/anchor";
import { ok } from "assert";
const { SystemProgram, Keypair } = web3;

const { AnchorProvider } = anchor;

describe("rustysolana", () => {
  const provider = AnchorProvider.env();
  setProvider(provider);


  const program = anchor.workspace.rustysolana;


  let _baseAccount;
  it("Creates a counter)", async () => {
    //Call the create function via RPC
    const baseAccount = anchor.web3.Keypair.generate();
    await program.rpc.create({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [baseAccount],
    });

    //Get the account data and check value of the counter
    const account = await program.account.baseAccount.fetch(
      baseAccount.publicKey
    );
    console.log("Count 0:", account.count.toString());
    ok(account.count.toString() === "0");
    _baseAccount = baseAccount;
  });

  it("Increments the counter", async () => {
    const baseAccount = _baseAccount;

    await program.rpc.increment({
      accounts: {
        baseAccount: baseAccount.publicKey,
      },
    });

    const account = await program.account.baseAccount.fetch(
      baseAccount.publicKey
    );
    console.log("Count 1:", account.count.toString());
    ok(account.count.toString() === "1");
  });
});
