// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { PublicKey, SystemProgram, Keypair } from "@solana/web3.js";
// import { TOKEN_PROGRAM_ID, createMint } from "@solana/spl-token";
// import { assert } from "chai";
// import { Nonce } from "../target/types/nonce";
// // import { Nonce } from "../target/types/nonce"; // Update with your actual program name

// describe("savings_program", () => {
//   const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);

//   const program = anchor.workspace.Nonce as Program<Nonce>;
  
//   // Test accounts
//   let mint: PublicKey;
//   let protocolVault: PublicKey;
//   let protocolUsdcVault: PublicKey;
//   let payer: Keypair;

//   before(async () => {
//     // Create test mint
//     payer = Keypair.generate();
//     await provider.connection.requestAirdrop(
//       payer.publicKey,
//       2 * anchor.web3.LAMPORTS_PER_SOL
//     );
    
//     mint = await createMint(
//       provider.connection,
//       payer,
//       payer.publicKey,
//       null,
//       6 // 6 decimals for USDC
//     );
//   });

//   describe("init_protocol_vault", () => {
//     it("initializes protocol vault correctly", async () => {
//       // Derive PDA for protocol vault
    //   const [protocolVaultPDA] = PublicKey.findProgramAddressSync(
    //     [Buffer.from("protocol")],
    //     program.programId
    //   );
//       protocolVault = protocolVaultPDA;

//       // Derive PDA for USDC vault
//       const [protocolUsdcVaultPDA] = PublicKey.findProgramAddressSync(
//         [Buffer.from("protocol")],
//         program.programId
//       );
//       protocolUsdcVault = protocolUsdcVaultPDA;

//       // Initialize protocol vault
//       await program.methods
//         .initializeProtocol()
//         .accounts({
//           mint,
//           payer: payer.publicKey,
//         //   protocolSolVault: protocolVault,
//         //   protocolUsdcVault,
//           tokenProgram: TOKEN_PROGRAM_ID,
//         //   systemProgram: SystemProgram.programId,
//         })
//         .signers([payer])
//         .rpc();

//       // Fetch and verify the initialized vault
//       const vaultAccount = await program.account.protocolVault.fetch(protocolVault);
      
//       assert.equal(vaultAccount.authority.toString(), payer.publicKey.toString());
//       assert.equal(vaultAccount.totalSolSaved.toNumber(), 0);
//       assert.equal(vaultAccount.totalUsdcSaved.toNumber(), 0);
//     //   assert(vaultAccount.lastUpdated > 0);
//       assert(vaultAccount.bump > 0);
//     });
//   });

//   describe("initialize_savings", () => {
//     it("initializes a time-locked savings account", async () => {
//       const name = "Test Savings";
//       const description = "Test Description";
//       const amount = new anchor.BN(1_000_000); // 1 SOL in lamports
//       const lockDuration = new anchor.BN(7 * 24 * 60 * 60); // 7 days in seconds

//       // Derive PDA for savings account
//       const [savingsAccountPDA] = PublicKey.findProgramAddressSync(
//         [
//           Buffer.from(name),
//           payer.publicKey.toBuffer(),
//           Buffer.from(description)
//         ],
//         program.programId
//       );

//       // Initialize savings account
//       await program.methods
//         .initializeSavings(
//           name,
//           description,
//           { timeLockedSavings: {} },
//           true, // is_sol
//           amount,
//           lockDuration,
//           null // unlock_price
//         )
//         .accounts({
//           signer: payer.publicKey,
//         //   systemProgram: SystemProgram.programId,
//         })
//         .signers([payer])
//         .rpc();

//       // Fetch and verify the initialized savings account
//       const savingsAccount = await program.account.savingsAccount.fetch(savingsAccountPDA);
      
//       assert.equal(savingsAccount.name, name);
//       assert.equal(savingsAccount.description, description);
//       assert.equal(savingsAccount.amount.toString(), amount.toString());
//       assert.equal(savingsAccount.owner.toString(), payer.publicKey.toString());
//       assert.equal(savingsAccount.isSol, true);
//       assert.equal(savingsAccount.lockDuration.toString(), lockDuration.toString());
//     //   assert(savingsAccount.createdAt > 0);
//     });

//     it("initializes a price-locked savings account", async () => {
//       const name = "Price Lock";
//       const description = "Price Locked Savings";
//       const amount = new anchor.BN(1_000_000);
//       const unlockPrice = new anchor.BN(2_000); // Example price target

//       // Derive PDA for savings account
//       const [savingsAccountPDA] = PublicKey.findProgramAddressSync(
//         [
//           Buffer.from(name),
//           payer.publicKey.toBuffer(),
//           Buffer.from(description)
//         ],
//         program.programId
//       );

//       // Initialize savings account
//       await program.methods
//         .initializeSavings(
//           name,
//           description,
//           { priceLockedSavings: {} },
//           false, // is_sol (using USDC)
//           amount,
//           null, // lock_duration
//           unlockPrice
//         )
//         .accounts({
//           signer: payer.publicKey,
//         //   savingsAccount: savingsAccountPDA,
//         //   systemProgram: SystemProgram.programId,
//         })
//         .signers([payer])
//         .rpc();

//       // Fetch and verify the initialized savings account
//       const savingsAccount = await program.account.savingsAccount.fetch(savingsAccountPDA);
      
//       assert.equal(savingsAccount.name, name);
//       assert.equal(savingsAccount.description, description);
//       assert.equal(savingsAccount.amount.toString(), amount.toString());
//       assert.equal(savingsAccount.owner.toString(), payer.publicKey.toString());
//       assert.equal(savingsAccount.isSol, false);
//       assert.equal(savingsAccount.unlockPrice.toString(), unlockPrice.toString());
//       assert.equal(savingsAccount.lockDuration.toString(), "0");
//     //   assert(savingsAccount.createdAt > 0);
//     });
//   });
// });