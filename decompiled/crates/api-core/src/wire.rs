//! Wire types / live chat frames.
//! EVIDENCE (binary): ChatFrame::{Hello,Chat,Receipt,FileOffer,FileChunk,...} serde variant names.

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiRequest {
    pub method: String,
    pub path: String,
    #[serde(default)]
    pub body: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub ok: bool,
    #[serde(default)]
    pub body: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Live data-channel frame enum (subset). EVIDENCE: binary ChatFrame variant strings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ChatFrame {
    Hello {
        #[serde(default)]
        profile: Option<Value>,
    },
    Chat {
        body: String,
        #[serde(default)]
        outbound_message_id: Option<String>,
    },
    Receipt {
        message_id: String,
        #[serde(default)]
        status: Option<String>,
    },
    FileOffer {
        name: String,
        #[serde(default)]
        mime: Option<String>,
        size: i64,
        ciphertext_len: i64,
    },
    FileChunk {
        offset: i64,
        data: String,
    },
    FileAck {
        bytes_received: i64,
    },
    FileComplete {},
    FileCancel {
        #[serde(default)]
        reason: Option<String>,
    },
    FileResume {},
    SessionRotate {},
    #[serde(other)]
    Unknown,
}
