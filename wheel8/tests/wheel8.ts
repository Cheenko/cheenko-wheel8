import * as anchor from "@coral-xyz/anchor";
import { AnchorProvider, Program, BN, web3 } from "@coral-xyz/anchor";
import { Wheel8 } from "../target/types/wheel8";

describe("wheel8", () => {
  const provider = AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.wheel8 as Program<Wheel8>;

  const payer = provider.wallet.publicKey;
  const systemProgram = web3.SystemProgram.programId;

  // We’ll keep the created config Keypair in memory for the spin test
  const configKey = web3.Keypair.generate();

  // choose your 8-slot multipliers (u16s). Edit if your program expects different values.
  const multipliers = [1, 2, 3, 4, 5, 6, 7, 8].map((n) => n as number);

  it("Is initialized!", async () => {
    // fund payer just in case
    try {
      await provider.connection.requestAirdrop(payer, 2 * web3.LAMPORTS_PER_SOL);
    } catch {}

    const tx = await program.methods
      .initialize(multipliers as any) // IDL: [u16;8]
      .accounts({
        config: configKey.publicKey,     // << brand new account (NOT a PDA)
        payer,
        systemProgram,
      })
      .signers([configKey])              // << crucial: config must sign when being created
      .rpc();

    console.log("✅ init tx:", tx);
    console.log("🧩 config pubkey:", configKey.publicKey.toBase58());
  });

  it("Spins the wheel (logs + optional event)", async () => {
    // Optional: event listener if you added #[event] SpinResult
    let listener: number | undefined;
    try {
      listener = await program.addEventListener("SpinResult", (e, slot) => {
        console.log("📡 SpinResult @ slot", slot, {
          player: (e as any).player?.toBase58?.() ?? (e as any).player,
          outcome: (e as any).outcome,
          payout: (e as any).payout?.toString?.() ?? (e as any).payout,
        });
      });
    } catch {}

    const seed = new BN(Date.now()); // adjust if your spin arg differs

    const tx = await program.methods
      .spin(seed)
      .accounts({
        config: configKey.publicKey,     // use the same account we just created
        player: payer,
      })
      .rpc();

    console.log("✅ spin tx:", tx);
    if (listener !== undefined) await program.removeEventListener(listener);
  });
});
