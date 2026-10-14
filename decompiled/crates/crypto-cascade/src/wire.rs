//! Cascade wire types.
//!
//! EVIDENCE (official golden + binary CascadeWire):
//! `{ v: 3, scheme: "cascade", finalCiphertext, outerParameters }` as base64 strings.
//! Binary also names optional `layers` / `originalSize` / `finalSize` — omitted when empty.

use crypto_core::HybridCiphertext;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Observed layer name tokens near outerParameters in binary.
pub const LAYER_NAMES: &[&str] = &["aes", "mls", "signal", "pqxdh", "rsa", "ml-kem"];

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CascadeEnvelope {
    pub v: u32,
    pub scheme: String,
    /// AES-GCM body (no tag). Tag + ML-KEM ct live in outerParameters.
    pub final_ciphertext: String,
    /// Base64 of BE length-prefixed nonce|tag|mlkem_ct.
    pub outer_parameters: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layers: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_size: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub final_size: Option<u64>,
}

#[derive(Debug, Clone)]
pub enum CiphertextKind {
    Cascade(CascadeEnvelope),
    Hybrid(HybridCiphertext),
    Unknown(Value),
}

pub fn detect_ciphertext(raw: &str) -> Result<CiphertextKind, serde_json::Error> {
    let v: Value = serde_json::from_str(raw)?;
    if let Some(obj) = v.as_object() {
        if obj.get("scheme").and_then(|x| x.as_str()) == Some("cascade")
            || (obj.get("v").and_then(|x| x.as_u64()) == Some(3)
                && obj.contains_key("finalCiphertext"))
        {
            return Ok(CiphertextKind::Cascade(serde_json::from_value(v)?));
        }
        if obj.contains_key("wrappedKey") || obj.contains_key("wrapped_key") {
            if let Ok(h) = serde_json::from_value::<HybridCiphertext>(v.clone()) {
                return Ok(CiphertextKind::Hybrid(h));
            }
            let wrapped = obj
                .get("wrappedKey")
                .or_else(|| obj.get("wrapped_key"))
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();
            let nonce = obj
                .get("nonce")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();
            let ciphertext = obj
                .get("ciphertext")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();
            return Ok(CiphertextKind::Hybrid(HybridCiphertext {
                wrapped_key: wrapped,
                nonce,
                ciphertext,
            }));
        }
    }
    Ok(CiphertextKind::Unknown(v))
}
