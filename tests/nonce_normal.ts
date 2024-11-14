import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Nonce } from "../target/types/nonce";
import { BN } from "bn.js";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import {
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
    createMint,
    getAccount,
    getAssociatedTokenAddressSync,
    getOrCreateAssociatedTokenAccount,
    mintTo,
} from '@solana/spl-token';
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from '@solana/web3.js';
import { assert, expect } from "chai";

describe("Nonce", async () => {
    const provider = anchor.AnchorProvider.env();

    anchor.setProvider(provider);

    const program = anchor.workspace.Nonce as Program<Nonce>;
    const user = anchor.web3.Keypair.generate();

    // let user: Keypair
    let user_ata: anchor.web3.PublicKey;
    let usdc_mint: anchor.web3.PublicKey;

    const wallet = provider.wallet as NodeWallet;

    const confirm = async (signature: string): Promise<string> => {
        const block = await provider.connection.getLatestBlockhash();
        await provider.connection.confirmTransaction({
            signature,
            ...block
        })
        return signature;
    }

    it("airdrop sol", async () => {
        const airdrop = await provider.connection.requestAirdrop(wallet.publicKey, 20 * anchor.web3.LAMPORTS_PER_SOL).then(confirm);
        console.log("\nAirdropped 20 sol to Wallet", airdrop);
        usdc_mint = await createMint(
            provider.connection,
            wallet.payer, wallet.payer.publicKey,
            wallet.payer.publicKey
            , 6);
        // user = anchor.web3.Keypair.generate();

        user_ata = (await getOrCreateAssociatedTokenAccount(provider.connection,wallet.payer,usdc_mint,wallet.payer.publicKey)).address;
        console.log("user ata is", user_ata.toBase58());

        // Mint 10,000 USDC (accounting for 6 decimals)
        await mintTo(
            provider.connection,
            wallet.payer,
            usdc_mint,
            user_ata,
            wallet.publicKey,
            10_000_000_000
        );
        const account = await getAccount(provider.connection, user_ata);
        // let decimalAmount = toDecimalAmount(account.amount,6)
        console.log(account);
    })
})