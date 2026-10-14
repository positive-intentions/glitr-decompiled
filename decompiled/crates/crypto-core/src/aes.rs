//! AES-256-GCM (12-byte nonce, 16-byte tag). EVIDENCE: docs Persistence.

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};

use crate::{random::random_bytes, CryptoError};

pub fn aes_encrypt(key32: &[u8], plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CryptoError> {
    if key32.len() != 32 {
        return Err(CryptoError::Msg("AES key must be 32 bytes".into()));
    }
    let cipher = Aes256Gcm::new_from_slice(key32)
        .map_err(|e| CryptoError::Msg(e.to_string()))?;
    let nonce_bytes = random_bytes(12);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ct = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| CryptoError::Msg(format!("aes encrypt: {e}")))?;
    Ok((nonce_bytes, ct))
}

pub fn aes_decrypt(key32: &[u8], nonce12: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if key32.len() != 32 || nonce12.len() != 12 {
        return Err(CryptoError::Msg("bad AES key/nonce lengths".into()));
    }
    let cipher = Aes256Gcm::new_from_slice(key32)
        .map_err(|e| CryptoError::Msg(e.to_string()))?;
    let nonce = Nonce::from_slice(nonce12);
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| CryptoError::Msg(format!("aes decrypt: {e}")))
}
