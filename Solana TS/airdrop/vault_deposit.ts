import { Connection, Keypair, SystemProgram, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js"
import { Program, Wallet, AnchorProvider, Address, BN } from "@project-serum/anchor"
import { WbaVault, IDL } from "./programs/wba_vault";
import wallet from "./wba-wallet.json"

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// get vaultstate public key
const vaultstate = new PublicKey("6MvYkDwoUJjGSLkpPhvkuDTwQAHzptZ4TZby7bhiEtB6")

// Create a devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), { commitment: "confirmed"});

// Create our program
const program = new Program<WbaVault>(IDL, "G7QyuwYPAcwrJ7p1S86gGbtVPt9A93vUyrMpc5xKEmoA" as Address, provider);

// Create vault auth PDA
const vault_auth_seed = [Buffer.from("auth"), vaultstate.toBuffer()];
const vault_auth = PublicKey.findProgramAddressSync(vault_auth_seed, program.programId)[0];

// create vault system program
const vault_seed = [Buffer.from("vault"), vault_auth.toBuffer()];
const vault = PublicKey.findProgramAddressSync(vault_seed, program.programId)[0];

// Execute our enrollment transaction
(async () => {
    try {
        const txhash = await program.methods
        .deposit(
            new BN(0.1*LAMPORTS_PER_SOL)
        )     
        .accounts({
            owner: keypair.publicKey,
            vaultState: vaultstate,
            vaultAuth: vault_auth,
            vault: vault,
            systemProgram: SystemProgram.programId,
        })
        .signers([
            keypair
        ]).rpc();
        console.log(`Success! Check out your TX here: 
        https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();