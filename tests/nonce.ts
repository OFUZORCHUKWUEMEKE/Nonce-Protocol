// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { Nonce } from "../target/types/nonce";
// import { expect } from "chai";
// import {
//   AccountInfoBytes,
//   AddedAccount,
//   BanksClient,
//   ProgramTestContext,
//   startAnchor,
//   BanksTransactionResultWithMeta
// } from "solana-bankrun";
// import { PublicKey, Transaction, Keypair, Connection, clusterApiUrl, TransactionInstruction } from "@solana/web3.js";
// import {
//   ACCOUNT_SIZE,
//   AccountLayout,
//   createMint,
//   getAssociatedTokenAddressSync,
//   getOrCreateAssociatedTokenAccount,
//   MintLayout,
//   TOKEN_2022_PROGRAM_ID,
//   type TOKEN_PROGRAM_ID,
// } from "@solana/spl-token";
// import { confirmTransaction, makeKeypairs } from '@solana-developers/helpers';
// import { BankrunProvider } from "anchor-bankrun";

// const IDL = require('../target/idl/nonce.json');
// const PROGRAM_ID = new PublicKey(IDL.address);

// const PROJECT_DIRECTORY = "";
// const USDC_DECIMALS = 6;



// describe("nonce", async () => {

//   const context = await startAnchor('', [{ name: 'nonce', programId: PROGRAM_ID }], []);
//   const provider = new BankrunProvider(context);

//   const connection = provider.connection;
//   const program = new anchor.Program<Nonce>(IDL, provider);

//   let usdc_mint: anchor.web3.PublicKey;

//   let user_ata: anchor.web3.PublicKey;

//   const payer = anchor.web3.Keypair.generate();


//   const confirm = async (signature: string): Promise<string> => {
//     const block = await provider.connection.getLatestBlockhash();
//     await provider.connection.confirmTransaction({
//       signature,
//       ...block
//     })
//     return signature;
//   }

//   it("Create user1 and user 2 accounts and the usdc mint and associated accounts for both users", async () => {
//     const airdrop = await provider.connection.requestAirdrop(payer.publicKey, 20 * anchor.web3.LAMPORTS_PER_SOL).then(confirm);
//     console.log("\nAirdropped 20 sol to user", airdrop);
//     usdc_mint = await createMint(provider.connection, payer, usdc_mint, payer.publicKey, 6);
//     user_ata = (await getOrCreateAssociatedTokenAccount(provider.connection,payer,usdc_mint,payer.publicKey)).address;

//     console.log("user_ata is",user_ata.toBase58());
//     expect(usdc_mint == typeof(PublicKey))

//   })



//   const program = anchor.workspace.Nonce as Program<Nonce>;

//   let context :ProgramTestContext;
//   let client :BanksClient;
//   let payer:Keypair;
//   let provider:BankrunProvider;
//   let program:Program<Nonce>;

//   before(async()=>{
//   const connection = new Connection(clusterApiUrl("testnet"));
//   const accountInfo = await connection.getAccountInfo(USDC_MINT_ADDRESS);
//   const usdcAccount :AddedAccount ={address:USDC_MINT_ADDRESS,info:accountInfo};

//   context = await startAnchor("", [], [usdcAccount]);
//   client = context.banksClient;
//   payer = context.payer;
//   provider = new BankrunProvider(context);
//   anchor.setProvider(provider);
//   program = new Program<Nonce>(IDL, provider)


//   })




//   async function createInstruction(
//     client: BanksClient,
//     payer: Keypair,
//     instruction: TransactionInstruction,
//     additionSigners: Keypair[] = []
//   ): Promise<BanksTransactionResultWithMeta> {
//     const tx = new Transaction();
//     const [latestBlockhash] = await client.getLatestBlockhash();
//     tx.recentBlockhash = latestBlockhash;
//     tx.add(instruction);
//     tx.feePayer = payer.publicKey;
//     tx.sign(payer, ...additionSigners);
//     return await client.tryProcessTransaction(tx);
//   }

//   it("Initialize Protocol", async () => {
//     Add your test here.     
//     console.log("testing one, two three")
 
//   });
// });