//! EVIDENCE: api_core::cascade_msg::{encrypt_to_contact,decrypt_from_contact,...}

use crypto_cascade::{decrypt_recipient, encrypt_recipient, LocalDecryptMaterial, RecipientMaterial};
use messaging_schema::Contact;

use crate::ApiError;

pub fn encrypt_to_contact(
    contact: &Contact,
    local_unused_bundles: Option<&str>,
    plaintext: &[u8],
) -> Result<String, ApiError> {
    let _ = local_unused_bundles;
    let pubkey = contact
        .peer_public_key
        .as_deref()
        .ok_or_else(|| ApiError::Msg("contact missing peerPublicKey".into()))?;
    let material = RecipientMaterial {
        peer_public_key_pem: pubkey.to_string(),
        peer_protocol_bundles: contact.peer_protocol_bundles.clone(),
    };
    Ok(encrypt_recipient(&material, plaintext)?)
}

pub fn decrypt_from_contact(
    private_key: &str,
    mlkem_private: Option<&[u8]>,
    ciphertext: &str,
) -> Result<Vec<u8>, ApiError> {
    Ok(decrypt_recipient(
        &LocalDecryptMaterial {
            private_key: private_key.to_string(),
            mlkem_private: mlkem_private.map(|s| s.to_vec()),
        },
        ciphertext,
    )?)
}

pub fn mlkem_private_from_protocol_secrets(protocol_secrets_json: &str) -> Result<Vec<u8>, ApiError> {
    let v: serde_json::Value = serde_json::from_str(protocol_secrets_json)
        .map_err(|e| ApiError::Msg(e.to_string()))?;
    let arr = v
        .get("mlkemPrivate")
        .and_then(|x| x.as_array())
        .ok_or_else(|| ApiError::Msg("protocolSecrets.mlkemPrivate missing".into()))?;
    let mut out = Vec::with_capacity(arr.len());
    for n in arr {
        let b = n
            .as_u64()
            .ok_or_else(|| ApiError::Msg("mlkemPrivate element not int".into()))?;
        if b > 255 {
            return Err(ApiError::Msg("mlkemPrivate byte out of range".into()));
        }
        out.push(b as u8);
    }
    Ok(out)
}
