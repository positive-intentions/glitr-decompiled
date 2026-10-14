//! WebRTC signaling helpers over the git `signaling/` collection.
//!
//! EVIDENCE: messaging.v10.graphql SignalingSession; binary ChatFrame variants;
//! contact fields webrtcEnabled/stunUrl; assets webrtc-peer*.js.

use chrono::Utc;
use messaging_schema::SignalingSession;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::wire::ChatFrame;
use crate::{ApiError, MailboxApi};

/// Default STUN — public STUN commonly used by the product UI.
pub const DEFAULT_STUN: &str = "stun:stun.l.google.com:19302";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalingPayload {
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sdp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub candidate: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frame: Option<ChatFrame>,
}

impl MailboxApi {
    pub fn post_signaling(
        &self,
        recipient_id: &str,
        role: &str,
        payload: &SignalingPayload,
    ) -> Result<SignalingSession, ApiError> {
        let me = self.profile_public()?;
        let id = Uuid::new_v4().to_string();
        let session = SignalingSession {
            id: id.clone(),
            recipient_id: recipient_id.to_string(),
            sender_id: me.id,
            role: Some(role.to_string()),
            ciphertext: serde_json::to_string(payload)?,
            updated_at: Some(Utc::now().timestamp()),
            from_device_id: None,
            to_device_id: None,
        };
        self.db
            .put_collection_row("signaling", &session.id, &session)?;
        self.db.commit("webrtc signaling")?;
        Ok(session)
    }

    pub fn list_signaling_for(&self, recipient_id: &str) -> Result<Vec<SignalingSession>, ApiError> {
        let mut out = vec![];
        for id in self.db.list_collection_ids("signaling")? {
            let s: SignalingSession = self.db.get_collection_row("signaling", &id)?;
            if s.recipient_id == recipient_id {
                out.push(s);
            }
        }
        out.sort_by_key(|s| s.updated_at.unwrap_or(0));
        Ok(out)
    }

    /// Pull peer signaling rows addressed to us from a peer mailbox workdir.
    pub fn poll_peer_signaling(
        &self,
        peer_workdir: impl AsRef<std::path::Path>,
    ) -> Result<Vec<SignalingSession>, ApiError> {
        let me = self.profile_public()?;
        let peer = git_core::GitRepo::open(peer_workdir)?;
        let mut created = vec![];
        for name in peer.list_dir("signaling")? {
            let Some(id) = name.strip_suffix(".json") else {
                continue;
            };
            let raw = peer.read_file(&format!("signaling/{name}"))?;
            let session: SignalingSession = serde_json::from_slice(&raw)?;
            if session.recipient_id != me.id {
                continue;
            }
            if self.db.git.exists(&format!("signaling/{id}.json")) {
                continue;
            }
            self.db.put_collection_row("signaling", id, &session)?;
            created.push(session);
        }
        if !created.is_empty() {
            self.db.commit("poll signaling")?;
        }
        Ok(created)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_frame_roundtrip_json() {
        let frame = ChatFrame::Chat {
            body: "hello webrtc".into(),
            outbound_message_id: None,
        };
        let s = serde_json::to_string(&frame).unwrap();
        let back: ChatFrame = serde_json::from_str(&s).unwrap();
        match back {
            ChatFrame::Chat { body, .. } => assert_eq!(body, "hello webrtc"),
            _ => panic!("expected Chat"),
        }
    }

    #[test]
    fn signaling_post_list() {
        let dir = tempfile::tempdir().unwrap();
        let api = MailboxApi::connect(dir.path(), "pw").unwrap();
        let me = api.setup_profile("A", "a").unwrap();
        let payload = SignalingPayload {
            kind: "offer".into(),
            sdp: Some("v=0".into()),
            candidate: None,
            frame: None,
        };
        let s = api.post_signaling(&me.id, "offer", &payload).unwrap();
        let listed = api.list_signaling_for(&me.id).unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].id, s.id);
    }
}
