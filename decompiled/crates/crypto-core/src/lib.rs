//! crypto-core primitives.
//!
//! EVIDENCE:
//! - docs Persistence: AES-256-GCM, RSA-OAEP-4096 SHA-256, Argon2id (19 MiB, t=2, p=1)
//! - binary: argon2-0.5.3, HybridCiphertext fields wrappedKey/nonce/ciphertext, typeName Argon2id
//! - embedded paths: crypto/crypto-core/src/{aes,cascade,password,random}.rs

mod aes;
mod hybrid;
mod mlkem_hybrid;
mod password;
mod random;

pub use aes::{aes_decrypt, aes_encrypt};
pub use hybrid::{decrypt_hybrid, encrypt_hybrid, generate_rsa_4096, HybridCiphertext};
pub use mlkem_hybrid::{
    b64_decode, b64_encode, decode_outer_parameters, encode_outer_parameters, generate_mlkem1024,
    mlkem_hybrid_decrypt, mlkem_hybrid_encrypt, MlKemKeyPairBytes, HKDF_INFO, MLKEM_CT_LEN,
    MLKEM_PK_LEN, MLKEM_SK_LEN,
};
pub use password::{
    seal_password, unseal_password, PasswordEnvelope, ARGON2_M_KIB, ARGON2_P, ARGON2_T,
};
pub use random::random_bytes;

#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("crypto: {0}")]
    Msg(String),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Rsa(#[from] rsa::Error),
}
