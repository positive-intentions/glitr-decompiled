//! At-rest password envelopes.
//!
//! EVIDENCE (docs Persistence): Argon2id 19 MiB, t=2, p=1 → AES-256-GCM.
//! EVIDENCE (official fixtures): JSON keys `version`,`kdf`,`aead`,`salt`,`nonce`,`ciphertext`
//! with integer enums `version=1`, `kdf=1` (Argon2id), `aead=1` (AES-256-GCM). No `m`/`t`/`p`
//! fields on the wire (params are fixed).

use argon2::{Algorithm, Argon2, Params, Version};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use serde::{Deserialize, Serialize};

use crate::{aes, random::random_bytes, CryptoError};

/// 19 MiB in KiB. EVIDENCE: docs Persistence + successful official unseal.
pub const ARGON2_M_KIB: u32 = 19 * 1024;
pub const ARGON2_T: u32 = 2;
pub const ARGON2_P: u32 = 1;

/// Official password-envelope integer enums (observed).
pub const ENVELOPE_VERSION: u32 = 1;
pub const KDF_ARGON2ID: u32 = 1;
pub const AEAD_AES256GCM: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordEnvelope {
    pub version: u32,
    pub kdf: u32,
    pub aead: u32,
    pub salt: String,
    pub nonce: String,
    pub ciphertext: String,
}

fn argon2_params(m: u32, t: u32, p: u32) -> Result<Params, CryptoError> {
    Params::new(m, t, p, Some(32)).map_err(|e| CryptoError::Msg(format!("argon2 params: {e}")))
}

fn derive_key(password: &str, salt: &[u8], m: u32, t: u32, p: u32) -> Result<[u8; 32], CryptoError> {
    let params = argon2_params(m, t, p)?;
    let argon = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = [0u8; 32];
    argon
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| CryptoError::Msg(format!("argon2: {e}")))?;
    Ok(key)
}

pub fn seal_password(password: &str, plaintext: &[u8]) -> Result<PasswordEnvelope, CryptoError> {
    let salt = random_bytes(16);
    let key = derive_key(password, &salt, ARGON2_M_KIB, ARGON2_T, ARGON2_P)?;
    let (nonce, ct) = aes::aes_encrypt(&key, plaintext)?;
    Ok(PasswordEnvelope {
        version: ENVELOPE_VERSION,
        kdf: KDF_ARGON2ID,
        aead: AEAD_AES256GCM,
        salt: B64.encode(salt),
        nonce: B64.encode(nonce),
        ciphertext: B64.encode(ct),
    })
}

pub fn unseal_password(password: &str, env: &PasswordEnvelope) -> Result<Vec<u8>, CryptoError> {
    if env.kdf != KDF_ARGON2ID {
        return Err(CryptoError::Msg(format!("unsupported kdf {}", env.kdf)));
    }
    if env.aead != AEAD_AES256GCM {
        return Err(CryptoError::Msg(format!("unsupported aead {}", env.aead)));
    }
    let salt = B64
        .decode(&env.salt)
        .map_err(|e| CryptoError::Msg(format!("salt b64: {e}")))?;
    let nonce = B64
        .decode(&env.nonce)
        .map_err(|e| CryptoError::Msg(format!("nonce b64: {e}")))?;
    let ct = B64
        .decode(&env.ciphertext)
        .map_err(|e| CryptoError::Msg(format!("ct b64: {e}")))?;
    let key = derive_key(password, &salt, ARGON2_M_KIB, ARGON2_T, ARGON2_P)?;
    aes::aes_decrypt(&key, &nonce, &ct)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_official_shape() {
        let env = seal_password("test-pass", b"hello mailbox").unwrap();
        assert_eq!(env.version, 1);
        assert_eq!(env.kdf, 1);
        assert_eq!(env.aead, 1);
        let json = serde_json::to_value(&env).unwrap();
        assert!(json.get("version").is_some());
        assert!(json.get("v").is_none());
        let pt = unseal_password("test-pass", &env).unwrap();
        assert_eq!(pt, b"hello mailbox");
    }
}
