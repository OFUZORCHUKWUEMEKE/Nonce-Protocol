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
    let connection = provider.connection;

    // Claude
    let protocolVault: PublicKey;
    let protocolUsdcVault: PublicKey;

    const Savings = {
        name: "Christmas",
        description: "Happy Christmas",
        savingsType: { timeLockedSavings: {} },
        is_sol: true,
        amount: 1000,
        lock_duration: 86400,
        unlock_price: 1000
    }

    // let user: Keypair
    let user_ata: anchor.web3.PublicKey;
    let usdc_mint: anchor.web3.PublicKey;

    const wallet = provider.wallet as NodeWallet;

    const [protocolPDA] = PublicKey.findProgramAddressSync(
        [Buffer.from("protocol")],
        program.programId
    );
    const [savingsPDA] = PublicKey.findProgramAddressSync([Buffer.from(Savings.name), provider.wallet.publicKey.toBuffer(), Buffer.from(Savings.description)], program.programId);

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


        user_ata = (await getOrCreateAssociatedTokenAccount(provider.connection, wallet.payer, usdc_mint, wallet.payer.publicKey)).address;
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
        //     const account = await getAccount(provider.connection, user_ata);
        //     console.log(account, "airdrop SOL");
    })

    it("Initialize Protocol", async () => {
        // Claude
        const [protocolVaultPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("protocol")],
            program.programId
          );
          protocolVault = protocolVaultPda;
      
          // Find PDA for protocol USDC vault
          const [protocolUsdcVaultPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("protocol")],
            program.programId
          );
          protocolUsdcVault = protocolUsdcVaultPda;
        try {
            const tx = await program.methods
                .initializeProtocol()
                .accounts({
                    mint: usdc_mint,
                    payer: provider.publicKey,
                    protocolSolVault: protocolVault,
                    protocolUsdcVault: protocolUsdcVault,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                });
                const account = await program.account.protocolVault.fetch(protocolVault);
                console.log(account)
        } catch (error) {
            console.log(error);
        }

        // const vaultAccount = await program.account.protocolVault.fetch(protocolPDA);
        // console.log(vaultAccount);
    })

    // it("Initialize Savings", async () => {
    //     try {
    //         await program.methods.initializeSavings(Savings.name, Savings.description, Savings.savingsType, true, new BN(1000), new BN(86400), new BN(1000)).accountsPartial({
    //             signer: provider.wallet.publicKey,
    //             savingsAccount: savingsPDA,
    //             systemProgram: anchor.web3.SystemProgram.programId,
    //         })
    //         const savingsAccount = await program.account.savingsAccount.fetch(savingsPDA);
    //         console.log(savingsAccount);
    //     } catch (error) {
    //         console.log(error)
    //     }

    // })
})