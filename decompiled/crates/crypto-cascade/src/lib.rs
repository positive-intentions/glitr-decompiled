//! Recipient cascade wire + product encrypt.
//!
//! EVIDENCE:
//! - docs: AES → MLS → Signal → PQXDH (inner), RSA hybrid → ML-KEM (outer); cascade `v: 3` `scheme: "cascade"`
//! - binary: finalCiphertext, outerParameters, ML-KEM-1024-AES-GCM
//! - fixtures/official/golden: official field names confirmed
//!
//! Reconstruction emits official outer wire + RSA hybrid inner. Full Signal/PQXDH/MLS
//! ratchet bytes remain UNKNOWN without Ghidra.

mod layers;
mod product;
mod wire;

pub use product::{
    decrypt_recipient, decrypt_recipient_rsa_only, encrypt_recipient, LocalDecryptMaterial,
    RecipientMaterial,
};
pub use wire::{detect_ciphertext, CascadeEnvelope, CiphertextKind, LAYER_NAMES};

#[derive(Debug, thiserror::Error)]
pub enum CascadeError {
    #[error("cascade: {0}")]
    Msg(String),
    #[error(transparent)]
    Crypto(#[from] crypto_core::CryptoError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
