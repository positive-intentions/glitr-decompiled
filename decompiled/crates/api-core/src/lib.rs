//! api-core in-process messaging API (reconstruction).
//!
//! EVIDENCE: docs Network; embedded api-core routes + cascade_msg + wire + schema.
//! HTTP route strings observed in binary (see evidence/http-routes.txt / strings-protocol).

mod cascade_msg;
mod routes;
mod webrtc;
mod wire;

pub use cascade_msg::{decrypt_from_contact, encrypt_to_contact, mlkem_private_from_protocol_secrets};
pub use routes::MailboxApi;
pub use webrtc::{SignalingPayload, DEFAULT_STUN};
pub use wire::{ApiRequest, ApiResponse, ChatFrame};

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("api: {0}")]
    Msg(String),
    #[error(transparent)]
    Db(#[from] db_core::DbError),
    #[error(transparent)]
    Git(#[from] git_core::GitError),
    #[error(transparent)]
    Cascade(#[from] crypto_cascade::CascadeError),
    #[error(transparent)]
    Crypto(#[from] crypto_core::CryptoError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
