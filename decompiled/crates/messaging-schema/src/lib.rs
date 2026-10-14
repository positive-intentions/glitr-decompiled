//! Messaging schema types reconstructed from the official TUI binary SDL.
//!
//! EVIDENCE (observed): `evidence/messaging.v10.graphql` extracted from
//! `glitr-tui-linux` (`schema @app(name: "messaging", version: 10)`).
//! Docs claimed v8; binary is authoritative for this investigation.

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const APP_NAME: &str = "messaging";
/// Observed in binary SDL extract (not the docs' v8 claim).
pub const SCHEMA_VERSION: u32 = 10;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaStamp {
    pub app: String,
    pub version: u32,
}

impl SchemaStamp {
    pub fn messaging_v10() -> Self {
        Self {
            app: APP_NAME.to_string(),
            version: SCHEMA_VERSION,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfilePublic {
    pub id: String,
    pub public_key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol_bundles: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retired_profile_ids: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileSecrets {
    pub id: String,
    pub display_name: String,
    pub handle: String,
    pub private_key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol_secrets: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub id: String,
    pub peer_profile_id: String,
    pub git_url: String,
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_polled_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clock_offset_ms: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_verify_error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peer_public_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peer_protocol_bundles: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_onion_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_mailbox: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peer_devices: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_enabled: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tor_enabled: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webrtc_enabled: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stun_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force_stun: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onion_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retired_profile_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutboundMessage {
    pub id: String,
    pub recipient_id: String,
    pub sender_id: String,
    pub ciphertext: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ack_read_receipt_ids: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadReceipt {
    pub id: String,
    pub message_id: String,
    pub sender_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InboxMessage {
    pub id: String,
    pub sender_id: String,
    pub body: String,
    pub outbound_message_id: String,
    pub from_contact_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub received_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment_mime: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment_data: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentMessage {
    pub id: String,
    pub contact_id: String,
    pub recipient_id: String,
    pub body: String,
    pub outbound_message_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment_mime: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment_data: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: String,
    pub title: String,
    pub member_profile_ids: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalingSession {
    pub id: String,
    pub recipient_id: String,
    pub sender_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    pub ciphertext: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_device_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolSession {
    pub id: String,
    pub contact_id: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupProtocolSession {
    pub id: String,
    pub group_id: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileTransfer {
    pub id: String,
    pub contact_id: String,
    pub direction: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    pub size: i64,
    pub ciphertext: String,
    pub bytes_done: i64,
    pub phase: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

/// Collections sealed with connect password (observed `@encrypted` in SDL).
pub const SEALED_DIRS: &[&str] = &[
    "contacts",
    "inbox",
    "sent",
    "groups",
    "protocol-sessions",
    "group-protocol-sessions",
    "file-transfers",
];

/// Peer-visible collections (docs + SDL without @encrypted).
pub const VISIBLE_DIRS: &[&str] = &["outbound", "readreceipts", "signaling", "devices"];

pub fn collection_dirs() -> &'static [&'static str] {
    &[
        "contacts",
        "devices",
        "outbound",
        "readreceipts",
        "inbox",
        "sent",
        "groups",
        "signaling",
        "protocol-sessions",
        "group-protocol-sessions",
        "file-transfers",
    ]
}

pub fn is_sealed_path(rel: &str) -> bool {
    SEALED_DIRS.iter().any(|d| rel.starts_with(d))
        || rel == "profile-secrets.json"
}

/// Opaque JSON for protocol bundles/secrets until cascade session format is fully recovered.
pub type JsonBlob = Value;
