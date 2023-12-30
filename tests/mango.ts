import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Mango } from "../target/types/mango";

const { SystemProgram } = anchor.web3;

describe("mango is ripe", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Mango as Program<Mango>;
  const storage_account = anchor.web3.Keypair.generate();
  const user = anchor.web3.Keypair.generate();
  const admin = anchor.web3.Keypair.generate();
  const payer = anchor.web3.Keypair.generate();
  const poolInfo = anchor.web3.Keypair.generate();
  const stakingToken = anchor.web3.Keypair.generate();
  const adminStakingWallet = anchor.web3.Keypair.generate();


  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts(
      {
        admin: admin.publicKey,
        payer: payer.publicKey,
        poolInfo: poolInfo.publicKey,
        stakingToken: stakingToken.publicKey,
        adminStakingWallet: adminStakingWallet.publicKey,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      }
    ).signers([payer]).rpc()

    console.log("Your transaction signature", tx);
  });
});

