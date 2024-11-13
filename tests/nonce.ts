import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Nonce } from "../target/types/nonce";
import { expect } from "chai";
import {
  AccountInfoBytes,
  AddedAccount,
  BanksClient,
  ProgramTestContext,
  startAnchor,
  BanksTransactionResultWithMeta
} from "solana-bankrun";
import { PublicKey, Transaction, Keypair, Connection, clusterApiUrl, TransactionInstruction } from "@solana/web3.js";
import {
  ACCOUNT_SIZE,
  AccountLayout,
  getAssociatedTokenAddressSync,
  MintLayout,
  TOKEN_PROGRAM_ID
} from "@solana/spl-token";

const IDL = require('../target/idl/nonce.json');

const PROJECT_DIRECTORY = "";
const USDC_DECIMALS = 6;
const USDC_MINT_ADDRESS = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";


describe("nonce", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Nonce as Program<Nonce>;

  async function createInstruction(
    client: BanksClient,
    payer: Keypair,
    instruction: TransactionInstruction,
    additionSigners: Keypair[]=[]
  ): Promise<BanksTransactionResultWithMeta> {
    const tx = new Transaction(); 
    const [latestBlockhash] = await client.getLatestBlockhash();
    tx.recentBlockhash = latestBlockhash;
    tx.add(instruction);
    tx.feePayer = payer.publicKey;
    tx.sign(payer, ...additionSigners);
    return await client.tryProcessTransaction(tx);
  }

  it("Is initialized!", async () => {
    // Add your test here.     
    console.log("testing one, two three")
  });
});