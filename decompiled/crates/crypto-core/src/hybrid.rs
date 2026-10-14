//! RSA-OAEP-4096 + AES-256-GCM hybrid (v1 fallback ciphertext).
//!
//! EVIDENCE:
//! - docs Data structures: `{ wrappedKey, nonce, ciphertext }` RSA-OAEP-4096 wraps AES key
//! - binary: HybridCiphertext, hybrid_to_json, encrypt_hybrid/decrypt_hybrid symbols
//! - binary error: "invalid wrappedKey encoding"

use base64::{engine::general_purpose::STANDARD as B64, Engine};
use rsa::{
    oaep::Oaep,
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding},
    RsaPrivateKey, RsaPublicKey,
};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::{aes, random::random_bytes, CryptoError};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HybridCiphertext {
    pub wrapped_key: String,
    pub nonce: String,
    pub ciphertext: String,
}

pub fn generate_rsa_4096() -> Result<(String, String), CryptoError> {
    let mut rng = rand::thread_rng();
    let privk = RsaPrivateKey::new(&mut rng, 4096)?;
    let pubk = RsaPublicKey::from(&privk);
    let priv_pem = privk
        .to_pkcs8_pem(LineEnding::LF)
        .map_err(|e| CryptoError::Msg(e.to_string()))?
        .to_string();
    let pub_pem = pubk
        .to_public_key_pem(LineEnding::LF)
        .map_err(|e| CryptoError::Msg(e.to_string()))?;
    Ok((pub_pem, priv_pem))
}

pub fn encrypt_hybrid(peer_public_pem: &str, plaintext: &[u8]) -> Result<HybridCiphertext, CryptoError> {
    let pubk = RsaPublicKey::from_public_key_pem(peer_public_pem)
        .map_err(|e| CryptoError::Msg(format!("parse peer pubkey: {e}")))?;
    let aes_key = random_bytes(32);
    let (nonce, ct) = aes::aes_encrypt(&aes_key, plaintext)?;
    let padding = Oaep::new::<Sha256>();
    let mut rng = rand::thread_rng();
    let wrapped = pubk
        .encrypt(&mut rng, padding, &aes_key)
        .map_err(|e| CryptoError::Msg(format!("rsa wrap: {e}")))?;
    Ok(HybridCiphertext {
        wrapped_key: B64.encode(wrapped),
        nonce: B64.encode(nonce),
        ciphertext: B64.encode(ct),
    })
}

pub fn decrypt_hybrid(private_pem: &str, hybrid: &HybridCiphertext) -> Result<Vec<u8>, CryptoError> {
    let privk = RsaPrivateKey::from_pkcs8_pem(private_pem)
        .map_err(|e| CryptoError::Msg(format!("parse privkey: {e}")))?;
    let wrapped = B64
        .decode(&hybrid.wrapped_key)
        .map_err(|e| CryptoError::Msg(format!("wrappedKey b64: {e}")))?;
    let nonce = B64
        .decode(&hybrid.nonce)
        .map_err(|e| CryptoError::Msg(format!("nonce b64: {e}")))?;
    let ct = B64
        .decode(&hybrid.ciphertext)
        .map_err(|e| CryptoError::Msg(format!("ciphertext b64: {e}")))?;
    let padding = Oaep::new::<Sha256>();
    let aes_key = privk
        .decrypt(padding, &wrapped)
        .map_err(|e| CryptoError::Msg(format!("rsa unwrap: {e}")))?;
    aes::aes_decrypt(&aes_key, &nonce, &ct)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hybrid_roundtrip() {
        let (pub_pem, priv_pem) = generate_rsa_4096().unwrap();
        let env = encrypt_hybrid(&pub_pem, b"secret message").unwrap();
        let pt = decrypt_hybrid(&priv_pem, &env).unwrap();
        assert_eq!(pt, b"secret message");
    }
}
