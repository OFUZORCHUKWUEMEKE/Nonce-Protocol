import * as anchor from "@coral-xyz/anchor";
import {
    mintTo,
    Account, getAccount, createMint, createAccount, getOrCreateAssociatedTokenAccount
} from "@solana/spl-token";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { Keypair, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { airdropIfRequired } from "@solana-developers/helpers";
import { Nonce } from '../target/types/nonce'


describe("Nonce Testing", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.Nonce as Program<Nonce>;
    const connection = provider.connection;
    const wallet = provider.wallet as anchor.Wallet;
    // const fakeWallet = Keypair.generate();

    let usdcMint: PublicKey;
    let recommendedVault = Keypair.generate();

    const DECIMALS = 1;
    const INITIAL_MINT_AMOUNT = 200;

    before(async () => {
        await airdropIfRequired(
            connection,
            provider.publicKey,
            1 * LAMPORTS_PER_SOL,
            0.5 * LAMPORTS_PER_SOL
        );

        usdcMint = await createMint(
            connection,
            wallet.payer,
            wallet.publicKey,
            null,
            DECIMALS
        )
    })

    it("Creation of Mint", async () => {
        console.log(usdcMint.toBase58(), "It worked")
    })

    
})