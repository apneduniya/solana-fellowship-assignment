use crate::models::message::{SignMessageRequest, SignMessageResponse, VerifyMessageRequest, VerifyMessageResponse};
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier, KEYPAIR_LENGTH};
use bs58;
use base64::{engine::general_purpose, Engine as _};

/// Signs a message with a secret key
pub fn sign_message(req: &SignMessageRequest) -> Result<SignMessageResponse, String> {
    let secret_bytes = bs58::decode(&req.secret).into_vec().map_err(|_| "Invalid base58 secret key")?;
    if secret_bytes.len() != KEYPAIR_LENGTH {
        return Err("Secret key must be 64 bytes (base58-encoded)".to_string());
    }
    let keypair = Keypair::from_bytes(&secret_bytes).map_err(|_| "Invalid secret key bytes")?;
    
    let signature = keypair.sign(req.message.as_bytes());

    Ok(SignMessageResponse {
        signature: general_purpose::STANDARD.encode(signature.to_bytes()),
        public_key: bs58::encode(keypair.public.to_bytes()).into_string(),
        message: req.message.clone(),
    })
}


/// Verifies a message with a public key and signature
pub fn verify_message(req: &VerifyMessageRequest) -> Result<VerifyMessageResponse, String> {
    let pubkey_bytes = bs58::decode(&req.pubkey).into_vec().map_err(|_| "Invalid base58 pubkey")?;
    let public_key = PublicKey::from_bytes(&pubkey_bytes).map_err(|_| "Invalid public key bytes")?;
    let signature_bytes = general_purpose::STANDARD.decode(&req.signature).map_err(|_| "Invalid base64 signature")?;
    let signature = Signature::from_bytes(&signature_bytes).map_err(|_| "Invalid signature bytes")?;

    let valid = public_key.verify(req.message.as_bytes(), &signature).is_ok();

    Ok(VerifyMessageResponse {
        valid,
        message: req.message.clone(),
        pubkey: req.pubkey.clone(),
    })
} 
