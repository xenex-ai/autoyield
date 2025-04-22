import * as anchor from "@project-serum/anchor";
import idl from "./idl/autoyield.json";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

async function main() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = new anchor.Program(idl, idl.metadata.address, provider);
  const authority = provider.wallet.publicKey;

  // Registry‑PDA
  const [registryPda] = await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("registry")],
    program.programId
  );

  // 1) Init Registry (nur einmal)
  await program.methods
    .initRegistry()
    .accounts({ registry: registryPda, authority })
    .rpc();

  // 2) Neue Strategie hinzufügen (z. B. Raydium‑Adapter)
  const raydiumProgramId = new anchor.web3.PublicKey("RAYDIUM11111111111111111111111111111111");
  await program.methods
    .addStrategy(raydiumProgramId)
    .accounts({ registry: registryPda, authority })
    .rpc();

  // 3) User‑Deposit
  // Ersetze diese Werte durch deine tatsächlichen Token- und Vault-Accounts
  const userTokenAccount = new anchor.web3.PublicKey("DEINE_TOKEN_ACCOUNT");
  const vault = new anchor.web3.PublicKey("STRATEGY_VAULT_ACCOUNT");

  await program.methods
    .deposit(new anchor.BN(1_000_000)) // 1 Token (6 Dezimals)
    .accounts({
      user: authority,
      userPos: await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("userpos"), authority.toBuffer(), raydiumProgramId.toBuffer()],
        program.programId
      )[0],
      strategy: raydiumProgramId,
      vault,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .rpc();

  // 4) Harvest All
  await program.methods
    .harvestAll()
    .accounts({ registry: registryPda, authority })
    .rpc({ skipPreflight: true });

  console.log("✅ AutoYield wurde ausgeführt!");
}

main().catch(err => {
  console.error(err);
});
