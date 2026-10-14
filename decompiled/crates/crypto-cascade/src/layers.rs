//! Layer ordering documentation as code.
//! EVIDENCE (docs Architecture / Cryptography): AES → MLS → Signal → PQXDH then RSA → ML-KEM.

#[allow(dead_code)]
pub const INNER_ORDER: &[&str] = &["aes", "mls", "signal", "pqxdh"];
#[allow(dead_code)]
pub const OUTER_ORDER: &[&str] = &["rsa", "ml-kem"];

/// UNKNOWN: byte-level composition of each layer in product encrypt_for_peer.
#[allow(dead_code)]
pub fn documented_stack() -> Vec<&'static str> {
    let mut v = INNER_ORDER.to_vec();
    v.extend_from_slice(OUTER_ORDER);
    v
}
