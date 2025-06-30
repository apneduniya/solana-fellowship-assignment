use solana_sdk::signature::{Keypair, Signer};
use bs58;
use crate::models::keypair::KeypairResponse;


/// Generates a new keypair
pub fn generate_keypair() -> KeypairResponse {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();
    
    KeypairResponse { pubkey, secret }
} 
