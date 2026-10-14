//! Product encrypt/decrypt used by api-core cascade_msg.
//!
//! EVIDENCE (docs): AES→MLS→Signal→PQXDH (inner) then RSA→ML-KEM (outer).
//! Without Ghidra, reconstruction uses a faithful **outer** ML-KEM hybrid and an
//! **inner** RSA-OAEP+AES hybrid payload (legacy HybridCiphertext). Official may
//! still reject until Signal/PQXDH/MLS layers match; recon↔recon uses this stack
//! with official wire field names (no `compat` / sidecar `hybrid`).

use base64::{engine::general_purpose::STANDARD as B64, Engine};
use crypto_core::{
    b64_decode, decrypt_hybrid, encrypt_hybrid, mlkem_hybrid_decrypt, mlkem_hybrid_encrypt,
    HybridCiphertext,
};

use crate::wire::{detect_ciphertext, CascadeEnvelope, CiphertextKind};
use crate::CascadeError;

#[derive(Debug, Clone)]
pub struct RecipientMaterial {
    /// RSA public key: PEM or standard base64 DER SPKI (official profile-public).
    pub peer_public_key_pem: String,
    /// Opaque protocolBundles JSON string from ProfilePublic.
    pub peer_protocol_bundles: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LocalDecryptMaterial {
    /// RSA private: PEM or base64 DER PKCS8.
    pub private_key: String,
    /// Raw ML-KEM-1024 expanded secret key (3168 bytes), if available.
    pub mlkem_private: Option<Vec<u8>>,
}

fn peer_mlkem_public(bundles_json: Option<&str>) -> Result<Option<Vec<u8>>, CascadeError> {
    let Some(raw) = bundles_json else {
        return Ok(None);
    };
    let v: serde_json::Value = serde_json::from_str(raw)?;
    let Some(mp) = v.get("mlkemPublic").and_then(|x| x.as_str()) else {
        return Ok(None);
    };
    Ok(Some(B64.decode(mp).map_err(|e| CascadeError::Msg(e.to_string()))?))
}

fn normalize_rsa_public(key: &str) -> Result<String, CascadeError> {
    let t = key.trim();
    if t.contains("BEGIN") {
        return Ok(t.to_string());
    }
    // Official stores base64 DER SPKI — wrap as PEM for rsa crate.
    let der = B64
        .decode(t)
        .map_err(|e| CascadeError::Msg(format!("peer publicKey b64: {e}")))?;
    let b64 = B64.encode(&der);
    let mut pem = String::from("-----BEGIN PUBLIC KEY-----\n");
    for chunk in b64.as_bytes().chunks(64) {
        pem.push_str(std::str::from_utf8(chunk).unwrap());
        pem.push('\n');
    }
    pem.push_str("-----END PUBLIC KEY-----\n");
    Ok(pem)
}

fn normalize_rsa_private(key: &str) -> Result<String, CascadeError> {
    let t = key.trim();
    if t.contains("BEGIN") {
        return Ok(t.to_string());
    }
    let der = B64
        .decode(t)
        .map_err(|e| CascadeError::Msg(format!("privateKey b64: {e}")))?;
    let b64 = B64.encode(&der);
    let mut pem = String::from("-----BEGIN PRIVATE KEY-----\n");
    for chunk in b64.as_bytes().chunks(64) {
        pem.push_str(std::str::from_utf8(chunk).unwrap());
        pem.push('\n');
    }
    pem.push_str("-----END PRIVATE KEY-----\n");
    Ok(pem)
}

/// Encrypt with RSA hybrid, then ML-KEM hybrid outer — official cascade JSON shape.
pub fn encrypt_recipient(material: &RecipientMaterial, plaintext: &[u8]) -> Result<String, CascadeError> {
    let mlkem_pk = peer_mlkem_public(material.peer_protocol_bundles.as_deref())?
        .ok_or_else(|| CascadeError::Msg("peer protocol bundles required (mlkemPublic)".into()))?;
    let rsa_pem = normalize_rsa_public(&material.peer_public_key_pem)?;
    let hybrid = encrypt_hybrid(&rsa_pem, plaintext)?;
    let inner = serde_json::to_vec(&hybrid)?;
    let (final_ct, outer) = mlkem_hybrid_encrypt(&mlkem_pk, &inner)?;
    let env = CascadeEnvelope {
        v: 3,
        scheme: "cascade".into(),
        final_ciphertext: B64.encode(&final_ct),
        outer_parameters: B64.encode(&outer),
        layers: None,
        original_size: Some(plaintext.len() as u64),
        final_size: Some(final_ct.len() as u64),
    };
    let s = serde_json::to_string(&env)?;
    // Guard: never emit reconstruction markers
    debug_assert!(!s.contains("compat"));
    debug_assert!(!s.contains("\"hybrid\""));
    Ok(s)
}

pub fn decrypt_recipient(
    material: &LocalDecryptMaterial,
    ciphertext_json: &str,
) -> Result<Vec<u8>, CascadeError> {
    match detect_ciphertext(ciphertext_json)? {
        CiphertextKind::Hybrid(h) => {
            let pem = normalize_rsa_private(&material.private_key)?;
            Ok(decrypt_hybrid(&pem, &h)?)
        }
        CiphertextKind::Cascade(env) => {
            let mlkem_sk = material.mlkem_private.as_ref().ok_or_else(|| {
                CascadeError::Msg("mlkemPrivate required to decrypt cascade v3".into())
            })?;
            let fc = b64_decode(&env.final_ciphertext)?;
            let op = b64_decode(&env.outer_parameters)?;
            let inner = mlkem_hybrid_decrypt(mlkem_sk, &fc, &op)?;
            // Inner should be HybridCiphertext JSON (reconstruction stack).
            if let Ok(h) = serde_json::from_slice::<HybridCiphertext>(&inner) {
                let pem = normalize_rsa_private(&material.private_key)?;
                return Ok(decrypt_hybrid(&pem, &h)?);
            }
            // Official may use opaque inner layers — return raw after outer peel for analysis.
            Err(CascadeError::Msg(format!(
                "cascade outer peeled ({} bytes) but inner is not HybridCiphertext JSON (Signal/PQXDH/MLS UNKNOWN)",
                inner.len()
            )))
        }
        CiphertextKind::Unknown(_) => Err(CascadeError::Msg("unrecognized ciphertext shape".into())),
    }
}

/// Back-compat helper used by older call sites that only have RSA PEM.
pub fn decrypt_recipient_rsa_only(private_pem: &str, ciphertext_json: &str) -> Result<Vec<u8>, CascadeError> {
    decrypt_recipient(
        &LocalDecryptMaterial {
            private_key: private_pem.to_string(),
            mlkem_private: None,
        },
        ciphertext_json,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_core::{generate_mlkem1024, generate_rsa_4096};

    #[test]
    fn cascade_official_shape_roundtrip() {
        let (pub_pem, priv_pem) = generate_rsa_4096().unwrap();
        let mlkem = generate_mlkem1024().unwrap();
        let bundles = serde_json::json!({
            "signal": {"reconstruction": true},
            "pqxdh": {"reconstruction": true},
            "mlkemPublic": B64.encode(&mlkem.public),
            "mlsKeyPackage": {"reconstruction": true}
        })
        .to_string();
        let mat = RecipientMaterial {
            peer_public_key_pem: pub_pem,
            peer_protocol_bundles: Some(bundles),
        };
        let ct = encrypt_recipient(&mat, b"hi cascade").unwrap();
        assert!(ct.contains("finalCiphertext"));
        assert!(ct.contains("outerParameters"));
        assert!(!ct.contains("compat"));
        assert!(!ct.contains("\"hybrid\""));
        let pt = decrypt_recipient(
            &LocalDecryptMaterial {
                private_key: priv_pem,
                mlkem_private: Some(mlkem.private),
            },
            &ct,
        )
        .unwrap();
        assert_eq!(pt, b"hi cascade");
    }

    #[test]
    fn structural_keys_match_official() {
        let (pub_pem, _) = generate_rsa_4096().unwrap();
        let mlkem = generate_mlkem1024().unwrap();
        let bundles = serde_json::json!({
            "mlkemPublic": B64.encode(&mlkem.public)
        })
        .to_string();
        let ct = encrypt_recipient(
            &RecipientMaterial {
                peer_public_key_pem: pub_pem,
                peer_protocol_bundles: Some(bundles),
            },
            b"x",
        )
        .unwrap();
        let v: serde_json::Value = serde_json::from_str(&ct).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("v").and_then(|x| x.as_u64()), Some(3));
        assert_eq!(obj.get("scheme").and_then(|x| x.as_str()), Some("cascade"));
        assert!(obj.contains_key("finalCiphertext"));
        assert!(obj.contains_key("outerParameters"));
        assert!(!obj.contains_key("compat"));
        assert!(!obj.contains_key("hybrid"));
        assert!(!obj.contains_key("ciphertext"));
    }
}
