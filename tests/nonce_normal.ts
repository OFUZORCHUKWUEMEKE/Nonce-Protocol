// import * as anchor from "@coral-xyz/anchor";
// import {
//     mintTo,
//     Account, getAccount, createMint, createAccount, getOrCreateAssociatedTokenAccount,
//     TOKEN_PROGRAM_ID
// } from "@solana/spl-token";
// import { Program } from "@coral-xyz/anchor";
// import { expect } from "chai";
// import { Keypair, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
// import { airdropIfRequired } from "@solana-developers/helpers";
// import { Nonce } from '../target/types/nonce'


// describe("Nonce Testing", () => {
//     const provider = anchor.AnchorProvider.env();
//     anchor.setProvider(provider);

//     const program = anchor.workspace.Nonce as Program<Nonce>;
//     const connection = provider.connection;
//     const wallet = provider.wallet as anchor.Wallet;
//     // const fakeWallet = Keypair.generate();

//     let usdcMint: PublicKey;
//     let recommendedVault = Keypair.generate();

//     const DECIMALS = 1;
//     const INITIAL_MINT_AMOUNT = 200;
//     let protocolUsdcVault = Keypair.generate();
//     let protocolUsdcBump;
//     let protocolSolVault: PublicKey;

//     before(async () => {
//         await airdropIfRequired(
//             connection,
//             provider.publicKey,
//             1 * LAMPORTS_PER_SOL,
//             0.5 * LAMPORTS_PER_SOL
//         );

//         usdcMint = await createMint(
//             connection,
//             wallet.payer,
//             wallet.publicKey,
//             null,
//             DECIMALS
//         );

//         // [protocolUsdcVault, protocolUsdcBump] = await PublicKey.findProgramAddressSync([Buffer.from("protocol")], program.programId)
//         [protocolSolVault] = await PublicKey.findProgramAddressSync([Buffer.from("protocol")], program.programId);
//         protocolUsdcBump = await createAccount(
//             connection,
//             wallet.payer,
//             usdcMint,
//             wallet.publicKey
//         )
//     })

//     it("Creation of Mint", async () => {
//         console.log(usdcMint.toBase58(), "It worked")
//     });

//     it("initialize protocol Savings", async () => {
//         try {
//             await program.methods.initializeProtocol().accountsPartial({
//                 mint: usdcMint,
//                 payer: wallet.publicKey,
//                 protocolSolVault: protocolSolVault,
//                 tokenProgram: TOKEN_PROGRAM_ID,
//                 protocolUsdcVault: protocolUsdcBump,
//                 systemProgram: anchor.web3.SystemProgram.programId
//             });

//             await mintTo(
//                 connection,
//                 wallet.payer,
//                 usdcMint,
//                 protocolUsdcBump.publicKey,
//                 wallet.payer,
//                 INITIAL_MINT_AMOUNT
//             );

//             const tokenAccount =  getAccount(connection,protocolUsdcBump);
//             const solvault = await program.account.protocolVault.fetch(protocolSolVault);
//             console.log(solvault);
//         } catch(error) {
//             console.log(error);
//         }
//     })


// })