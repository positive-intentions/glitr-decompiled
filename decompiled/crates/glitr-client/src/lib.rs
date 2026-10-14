//! glitr-client shared session/adapter façade.
//!
//! EVIDENCE: docs Network; embedded glitr-client/src/{handle,ops,connect,proxy,tor*,webrtc*,ui_adapter,web_storage}.rs
//! Git mailbox is P0. WebRTC signaling is available via api-core (P1).
//! Tor onion path remains deferred.

mod connect;
mod handle;

pub use connect::SavedConnect;
pub use handle::ApiHandle;
