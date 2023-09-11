import { Keypair, Commitment, Connection } from '@solana/web3.js'
import { createMint } from "@solana/spl-token"
import wallet from '../dev-wallet.json'

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet))

const commitment: Commitment = 'confirmed'
const connection = new Connection('https://api.devnet.solana.com', commitment);

(async () => {
    try {
        const mint = await createMint(connection, keypair, keypair.publicKey, keypair.publicKey, 6)
        console.log( 
            `Token Mint: https://explorer.solana.com/address/${mint}?cluster=devnet`
          );
    } catch (error) {
        console.log(error)
    }
})()