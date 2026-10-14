//! ML-KEM-1024 hybrid AES-256-GCM (outer cascade layer).
//!
//! EVIDENCE:
//! - docs Cryptography: encapsulate → HKDF-SHA256 info `ML-KEM-1024-AES-GCM` → AES-256-GCM
//! - binary string `ML-KEM-1024-AES-GCM`; libcrux-ml-kem 0.0.10
//! - official outerParameters: BE u32 lengths → nonce(12) + tag(16) + mlkem_ct(1568)
//! - finalCiphertext is AES-GCM body without trailing tag (tag lives in outerParameters)

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use hkdf::Hkdf;
use libcrux_ml_kem::mlkem1024::{self, MlKem1024Ciphertext, MlKem1024PrivateKey, MlKem1024PublicKey};
use libcrux_ml_kem::{ENCAPS_SEED_SIZE, KEY_GENERATION_SEED_SIZE};
use sha2::Sha256;

use crate::{random::random_bytes, CryptoError};

fn fill_random(buf: &mut [u8]) -> Result<(), CryptoError> {
    use rand::RngCore;
    rand::thread_rng().fill_bytes(buf);
    Ok(())
}

pub const MLKEM_PK_LEN: usize = 1568;
pub const MLKEM_SK_LEN: usize = 3168;
pub const MLKEM_CT_LEN: usize = 1568;
pub const HKDF_INFO: &[u8] = b"ML-KEM-1024-AES-GCM";

#[derive(Debug, Clone)]
pub struct MlKemKeyPairBytes {
    pub public: Vec<u8>,
    pub private: Vec<u8>,
}

pub fn generate_mlkem1024() -> Result<MlKemKeyPairBytes, CryptoError> {
    let mut seed = [0u8; KEY_GENERATION_SEED_SIZE];
    fill_random(&mut seed)?;
    let kp = mlkem1024::generate_key_pair(seed);
    Ok(MlKemKeyPairBytes {
        public: kp.public_key().as_slice().to_vec(),
        private: kp.private_key().as_slice().to_vec(),
    })
}

fn derive_aes_key(shared_secret: &[u8]) -> Result<[u8; 32], CryptoError> {
    let hk = Hkdf::<Sha256>::new(None, shared_secret);
    let mut okm = [0u8; 32];
    hk.expand(HKDF_INFO, &mut okm)
        .map_err(|_| CryptoError::Msg("hkdf expand".into()))?;
    Ok(okm)
}

fn write_u32_be(buf: &mut Vec<u8>, n: u32) {
    buf.extend_from_slice(&n.to_be_bytes());
}

/// Build official-style outerParameters blob.
pub fn encode_outer_parameters(nonce12: &[u8], tag16: &[u8], mlkem_ct: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(4 + 12 + 4 + 16 + 4 + mlkem_ct.len());
    write_u32_be(&mut out, nonce12.len() as u32);
    out.extend_from_slice(nonce12);
    write_u32_be(&mut out, tag16.len() as u32);
    out.extend_from_slice(tag16);
    write_u32_be(&mut out, mlkem_ct.len() as u32);
    out.extend_from_slice(mlkem_ct);
    out
}

pub fn decode_outer_parameters(raw: &[u8]) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), CryptoError> {
    let mut i = 0usize;
    let mut read = |expected: Option<usize>| -> Result<Vec<u8>, CryptoError> {
        if i + 4 > raw.len() {
            return Err(CryptoError::Msg("outerParameters truncated".into()));
        }
        let len = u32::from_be_bytes(raw[i..i + 4].try_into().unwrap()) as usize;
        i += 4;
        if let Some(e) = expected {
            if len != e {
                return Err(CryptoError::Msg(format!(
                    "outerParameters chunk len {len} != {e}"
                )));
            }
        }
        if i + len > raw.len() {
            return Err(CryptoError::Msg("outerParameters chunk OOB".into()));
        }
        let slice = raw[i..i + len].to_vec();
        i += len;
        Ok(slice)
    };
    let nonce = read(Some(12))?;
    let tag = read(Some(16))?;
    let mlkem_ct = read(Some(MLKEM_CT_LEN))?;
    if i != raw.len() {
        return Err(CryptoError::Msg("outerParameters trailing bytes".into()));
    }
    Ok((nonce, tag, mlkem_ct))
}

/// Encrypt plaintext to peer ML-KEM public key (1568 bytes).
/// Returns (final_ciphertext_without_tag, outer_parameters_bytes).
pub fn mlkem_hybrid_encrypt(
    peer_mlkem_pk: &[u8],
    plaintext: &[u8],
) -> Result<(Vec<u8>, Vec<u8>), CryptoError> {
    if peer_mlkem_pk.len() != MLKEM_PK_LEN {
        return Err(CryptoError::Msg(format!(
            "mlkem pk len {} != {MLKEM_PK_LEN}",
            peer_mlkem_pk.len()
        )));
    }
    let pk = MlKem1024PublicKey::try_from(peer_mlkem_pk)
        .map_err(|_| CryptoError::Msg("bad mlkem public key".into()))?;
    let mut enc_seed = [0u8; ENCAPS_SEED_SIZE];
    fill_random(&mut enc_seed)?;
    let (ct, ss) = mlkem1024::encapsulate(&pk, enc_seed);
    let aes_key = derive_aes_key(ss.as_ref())?;
    let cipher = Aes256Gcm::new_from_slice(&aes_key).map_err(|e| CryptoError::Msg(e.to_string()))?;
    let nonce_bytes = random_bytes(12);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let sealed = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| CryptoError::Msg(format!("aes encrypt: {e}")))?;
    // aes-gcm crate returns ciphertext || tag
    if sealed.len() < 16 {
        return Err(CryptoError::Msg("aes sealed too short".into()));
    }
    let (body, tag) = sealed.split_at(sealed.len() - 16);
    let outer = encode_outer_parameters(&nonce_bytes, tag, ct.as_slice());
    Ok((body.to_vec(), outer))
}

pub fn mlkem_hybrid_decrypt(
    private_mlkem_sk: &[u8],
    final_ciphertext: &[u8],
    outer_parameters: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    if private_mlkem_sk.len() != MLKEM_SK_LEN {
        return Err(CryptoError::Msg(format!(
            "mlkem sk len {} != {MLKEM_SK_LEN}",
            private_mlkem_sk.len()
        )));
    }
    let (nonce, tag, mlkem_ct) = decode_outer_parameters(outer_parameters)?;
    let sk = MlKem1024PrivateKey::try_from(private_mlkem_sk)
        .map_err(|_| CryptoError::Msg("bad mlkem private key".into()))?;
    let ct = MlKem1024Ciphertext::try_from(mlkem_ct.as_slice())
        .map_err(|_| CryptoError::Msg("bad mlkem ciphertext".into()))?;
    let ss = mlkem1024::decapsulate(&sk, &ct);
    let aes_key = derive_aes_key(ss.as_ref())?;
    let cipher = Aes256Gcm::new_from_slice(&aes_key).map_err(|e| CryptoError::Msg(e.to_string()))?;
    let nonce = Nonce::from_slice(&nonce);
    let mut sealed = Vec::with_capacity(final_ciphertext.len() + tag.len());
    sealed.extend_from_slice(final_ciphertext);
    sealed.extend_from_slice(&tag);
    cipher
        .decrypt(nonce, sealed.as_ref())
        .map_err(|e| CryptoError::Msg(format!("aes decrypt: {e}")))
}

/// Convenience: base64 wire helpers used by cascade.
pub fn b64_encode(bytes: &[u8]) -> String {
    B64.encode(bytes)
}

pub fn b64_decode(s: &str) -> Result<Vec<u8>, CryptoError> {
    B64.decode(s)
        .map_err(|e| CryptoError::Msg(format!("b64: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mlkem_hybrid_roundtrip() {
        let kp = generate_mlkem1024().unwrap();
        let (fc, op) = mlkem_hybrid_encrypt(&kp.public, b"OFFICIAL_GOLDEN_1").unwrap();
        let (n, t, c) = decode_outer_parameters(&op).unwrap();
        assert_eq!(n.len(), 12);
        assert_eq!(t.len(), 16);
        assert_eq!(c.len(), MLKEM_CT_LEN);
        let pt = mlkem_hybrid_decrypt(&kp.private, &fc, &op).unwrap();
        assert_eq!(pt, b"OFFICIAL_GOLDEN_1");
    }
}
