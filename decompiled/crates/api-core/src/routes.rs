//! Hand-written mailbox operations matching observed route names.
//! EVIDENCE: /auth/login, /messages/send, /messages/poll, /contacts/, /profile/setup, ...

use std::path::Path;

use base64::{engine::general_purpose::STANDARD as B64, Engine};
use chrono::Utc;
use crypto_core::{generate_mlkem1024, generate_rsa_4096};
use db_core::MailboxDb;
use messaging_schema::{
    Contact, InboxMessage, OutboundMessage, ProfilePublic, ProfileSecrets, SentMessage,
};
use rsa::pkcs8::DecodePrivateKey;
use rsa::{RsaPrivateKey, RsaPublicKey};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::cascade_msg::{
    decrypt_from_contact, encrypt_to_contact, mlkem_private_from_protocol_secrets,
};
use crate::ApiError;

pub struct MailboxApi {
    pub db: MailboxDb,
}

fn rsa_public_der_b64(priv_pem: &str) -> Result<String, ApiError> {
    let privk = RsaPrivateKey::from_pkcs8_pem(priv_pem)
        .map_err(|e| ApiError::Msg(format!("rsa priv: {e}")))?;
    let pubk = RsaPublicKey::from(&privk);
    use rsa::pkcs8::EncodePublicKey;
    let der = pubk
        .to_public_key_der()
        .map_err(|e| ApiError::Msg(e.to_string()))?;
    Ok(B64.encode(der.as_bytes()))
}

fn rsa_private_der_b64(priv_pem: &str) -> Result<String, ApiError> {
    use rsa::pkcs8::EncodePrivateKey;
    let privk = RsaPrivateKey::from_pkcs8_pem(priv_pem)
        .map_err(|e| ApiError::Msg(format!("rsa priv: {e}")))?;
    let der = privk
        .to_pkcs8_der()
        .map_err(|e| ApiError::Msg(e.to_string()))?;
    Ok(B64.encode(der.as_bytes()))
}

/// Minimal protocolBundles / protocolSecrets matching official key names.
/// EVIDENCE: fixtures/official + official-secrets-keytree.json.
/// Inner Signal/PQXDH/MLS material is placeholder-shaped for identity; ML-KEM is real.
fn build_protocol_material() -> Result<(String, String, Vec<u8>), ApiError> {
    let mlkem = generate_mlkem1024()?;
    let pk_b64 = B64.encode(&mlkem.public);
    let sk_ints: Vec<u8> = mlkem.private.clone();
    // Official stores mlkemPrivate as JSON array of ints.
    let sk_json: Vec<serde_json::Value> = sk_ints.iter().map(|b| serde_json::json!(*b)).collect();
    let pk_json: Vec<serde_json::Value> = mlkem
        .public
        .iter()
        .map(|b| serde_json::json!(*b))
        .collect();

    // Placeholder classical/PQ handshake material (structure only).
    // UNKNOWN: exact official byte encodings for signal/pqxdh/mls beyond key names.
    let mut seed = Sha256::digest(b"glitr-recon-signal-seed");
    let mut take32 = || -> Vec<u8> {
        let out = seed.to_vec();
        seed = Sha256::digest(&out);
        out
    };
    let sig_id = take32();
    let sig_sign = take32();

    let bundles = serde_json::json!({
        "signal": {
            "identityPublic": B64.encode(&sig_id),
            "identitySignPublic": B64.encode(&sig_sign),
            "signedPrekeyId": 1,
            "signedPrekeyPublic": B64.encode(&take32()),
            "signedPrekeySignature": B64.encode(&take32()),
            "oneTimePrekeyId": 1,
            "oneTimePrekeyPublic": B64.encode(&take32())
        },
        "pqxdh": {
            "identitySignPublic": B64.encode(&take32()),
            "signedPrekeyId": 1,
            "signedPrekeyPublic": B64.encode(&take32()),
            "signedPrekeySignature": B64.encode(&take32()),
            "pqPrekeyId": 1,
            "pqPrekeyKind": "ML-KEM-1024",
            "pqPrekeyPublic": pk_b64,
            "pqPrekeySignature": B64.encode(&take32()),
            "oneTimePrekeyId": 1,
            "oneTimePrekeyPublic": B64.encode(&take32()),
            "identityPublic": B64.encode(&take32())
        },
        "mlkemPublic": pk_b64,
        "mlsKeyPackage": {
            "key_package_bytes": B64.encode(&take32()),
            "leaf_node_key": B64.encode(&take32()),
            "expiration": 0
        }
    });

    let secrets = serde_json::json!({
        "signalIdentity": { "public_key": sig_id, "private_key": take32() },
        "signalSign": { "public_key": sig_sign, "private_key": take32() },
        "signalSignedPrekey": { "public_key": take32(), "private_key": take32() },
        "signalSignedPrekeyId": 1,
        "signalSignedPrekeySignature": take32(),
        "signalOneTimePrekey": { "public_key": take32(), "private_key": take32() },
        "signalOtpkPool": [],
        "pqxdhIdentity": {
            "dh": { "public_key": take32(), "private_key": take32() },
            "sign": { "public_key": take32(), "private_key": take32() }
        },
        "pqxdhSignedPrekey": { "id": 1, "key_pair": { "public_key": take32(), "private_key": take32() }, "signature": take32() },
        "pqxdhPqPrekey": { "id": 1, "kind": "ML-KEM-1024", "public_key": pk_json, "private_key": sk_json.clone(), "signature": take32() },
        "pqxdhOtpk": { "id": 1, "key_pair": { "public_key": take32(), "private_key": take32() } },
        "pqxdhOtpkPool": [],
        "pqxdhOtpkNextId": 2,
        "mlkemPrivate": sk_json,
        "mlkemPublic": mlkem.public.iter().map(|b| serde_json::json!(*b)).collect::<Vec<_>>(),
        "mlsIdentity": { "label": "recon", "signing_secret": take32(), "signing_public": take32(), "key_packages": [] },
        "mlsKeyPackage": { "key_package_bytes": take32(), "leaf_node_key": take32(), "expiration": 0 }
    });

    Ok((
        bundles.to_string(),
        secrets.to_string(),
        mlkem.private,
    ))
}

impl MailboxApi {
    pub fn connect(path: impl AsRef<Path>, password: &str) -> Result<Self, ApiError> {
        let db = if path.as_ref().join("schemaVersion.json").exists() {
            MailboxDb::open(path, Some(password))?
        } else {
            MailboxDb::bootstrap(path, Some(password))?
        };
        Ok(Self { db })
    }

    /// /profile/setup — create identity keys if missing.
    pub fn setup_profile(
        &self,
        display_name: &str,
        handle: &str,
    ) -> Result<ProfilePublic, ApiError> {
        if self.db.git.exists("profile-public.json") {
            return Ok(self.db.get_json("profile-public.json")?);
        }
        let (_pub_pem, priv_pem) = generate_rsa_4096()?;
        let public_key = rsa_public_der_b64(&priv_pem)?;
        let private_key = rsa_private_der_b64(&priv_pem)?;
        let (bundles, secrets_json, _) = build_protocol_material()?;
        let id = Uuid::new_v4().to_string();
        let public = ProfilePublic {
            id: id.clone(),
            public_key,
            protocol_bundles: Some(bundles),
            retired_profile_ids: None,
        };
        let secrets = ProfileSecrets {
            id: id.clone(),
            display_name: display_name.to_string(),
            handle: handle.to_string(),
            private_key,
            protocol_secrets: Some(secrets_json),
        };
        self.db.put_json("profile-public.json", &public)?;
        self.db.put_json("profile-secrets.json", &secrets)?;
        self.db.commit("profile setup")?;
        Ok(public)
    }

    pub fn profile_public(&self) -> Result<ProfilePublic, ApiError> {
        Ok(self.db.get_json("profile-public.json")?)
    }

    pub fn profile_secrets(&self) -> Result<ProfileSecrets, ApiError> {
        Ok(self.db.get_json("profile-secrets.json")?)
    }

    pub fn add_contact(
        &self,
        peer_profile_id: &str,
        git_url: &str,
        display_name: &str,
        peer_public_key: Option<&str>,
        token: Option<&str>,
    ) -> Result<Contact, ApiError> {
        self.add_contact_full(
            peer_profile_id,
            git_url,
            display_name,
            peer_public_key,
            None,
            token,
        )
    }

    pub fn add_contact_full(
        &self,
        peer_profile_id: &str,
        git_url: &str,
        display_name: &str,
        peer_public_key: Option<&str>,
        peer_protocol_bundles: Option<&str>,
        token: Option<&str>,
    ) -> Result<Contact, ApiError> {
        // If peer is a local path, load live public profile (avoids stale ids).
        let mut peer_id = peer_profile_id.to_string();
        let mut peer_pk = peer_public_key.map(|s| s.to_string());
        let mut peer_bundles = peer_protocol_bundles.map(|s| s.to_string());
        let peer_path = Path::new(git_url);
        if peer_path.join("profile-public.json").exists() {
            let peer_pub: ProfilePublic = {
                let raw = std::fs::read(peer_path.join("profile-public.json"))
                    .map_err(|e| ApiError::Msg(e.to_string()))?;
                serde_json::from_slice(&raw)?
            };
            peer_id = peer_pub.id;
            if peer_pk.is_none() {
                peer_pk = Some(peer_pub.public_key);
            }
            if peer_bundles.is_none() {
                peer_bundles = peer_pub.protocol_bundles;
            }
        }

        let id = Uuid::new_v4().to_string();
        let contact = Contact {
            id: id.clone(),
            peer_profile_id: peer_id,
            git_url: git_url.to_string(),
            display_name: display_name.to_string(),
            username: None,
            token: token.map(|s| s.to_string()),
            last_polled_at: None,
            clock_offset_ms: None,
            connection_status: Some("added".into()),
            last_verify_error: None,
            peer_public_key: peer_pk,
            peer_protocol_bundles: peer_bundles,
            local_onion_address: None,
            git_mailbox: Some(git_url.to_string()),
            peer_devices: None,
            git_enabled: Some("true".into()),
            tor_enabled: None,
            webrtc_enabled: Some("true".into()),
            stun_url: Some("stun:stun.l.google.com:19302".into()),
            force_stun: None,
        };
        self.db.put_collection_row("contacts", &id, &contact)?;
        self.db.commit("add contact")?;
        Ok(contact)
    }

    pub fn list_contacts(&self) -> Result<Vec<Contact>, ApiError> {
        let mut out = vec![];
        for id in self.db.list_collection_ids("contacts")? {
            out.push(self.db.get_collection_row("contacts", &id)?);
        }
        Ok(out)
    }

    /// /messages/send — write outbound (visible ciphertext) + sent (sealed plaintext).
    pub fn send_text(&self, contact_id: &str, body: &str) -> Result<OutboundMessage, ApiError> {
        let contact: Contact = self.db.get_collection_row("contacts", contact_id)?;
        let me = self.profile_public()?;
        let ciphertext = encrypt_to_contact(&contact, None, body.as_bytes())?;
        let outbound_id = Uuid::new_v4().to_string();
        let now = Utc::now().timestamp();
        let outbound = OutboundMessage {
            id: outbound_id.clone(),
            recipient_id: contact.peer_profile_id.clone(),
            sender_id: me.id.clone(),
            ciphertext,
            body: None,
            created_at: Some(now),
            ack_read_receipt_ids: None,
            group_id: None,
        };
        let sent = SentMessage {
            id: Uuid::new_v4().to_string(),
            contact_id: contact.id.clone(),
            recipient_id: contact.peer_profile_id.clone(),
            body: body.to_string(),
            outbound_message_id: outbound_id.clone(),
            created_at: Some(now),
            read_at: None,
            attachment_name: None,
            attachment_mime: None,
            attachment_size: None,
            attachment_data: None,
            group_id: None,
        };
        self.db
            .put_collection_row("outbound", &outbound.id, &outbound)?;
        self.db.put_collection_row("sent", &sent.id, &sent)?;
        self.db.commit("send message")?;
        Ok(outbound)
    }

    /// Ingest peer outbound addressed to us into local inbox (git poll simulation).
    pub fn ingest_peer_outbound_dir(
        &self,
        peer_workdir: impl AsRef<Path>,
        from_contact_id: &str,
    ) -> Result<Vec<InboxMessage>, ApiError> {
        let me = self.profile_public()?;
        let secrets = self.profile_secrets()?;
        let mlkem = secrets
            .protocol_secrets
            .as_deref()
            .map(mlkem_private_from_protocol_secrets)
            .transpose()?;
        let peer = git_core::GitRepo::open(peer_workdir)?;
        let mut created = vec![];
        for name in peer.list_dir("outbound")? {
            let Some(_id) = name.strip_suffix(".json") else {
                continue;
            };
            let raw = peer.read_file(&format!("outbound/{name}"))?;
            let outbound: OutboundMessage = serde_json::from_slice(&raw)?;
            if outbound.recipient_id != me.id {
                continue;
            }
            let already = self.db.list_collection_ids("inbox")?.iter().any(|iid| {
                self.db
                    .get_collection_row::<InboxMessage>("inbox", iid)
                    .map(|m| m.outbound_message_id == outbound.id)
                    .unwrap_or(false)
            });
            if already {
                continue;
            }
            let pt = decrypt_from_contact(
                &secrets.private_key,
                mlkem.as_deref(),
                &outbound.ciphertext,
            )?;
            let body = String::from_utf8(pt).map_err(|e| ApiError::Msg(e.to_string()))?;
            let inbox = InboxMessage {
                id: Uuid::new_v4().to_string(),
                sender_id: outbound.sender_id.clone(),
                body,
                outbound_message_id: outbound.id.clone(),
                from_contact_id: from_contact_id.to_string(),
                received_at: Some(Utc::now().timestamp()),
                created_at: outbound.created_at,
                attachment_name: None,
                attachment_mime: None,
                attachment_size: None,
                attachment_data: None,
                group_id: outbound.group_id.clone(),
            };
            self.db.put_collection_row("inbox", &inbox.id, &inbox)?;
            created.push(inbox);
        }
        if !created.is_empty() {
            self.db.commit("poll ingest")?;
        }
        Ok(created)
    }

    pub fn list_inbox(&self) -> Result<Vec<InboxMessage>, ApiError> {
        let mut out = vec![];
        for id in self.db.list_collection_ids("inbox")? {
            out.push(self.db.get_collection_row("inbox", &id)?);
        }
        Ok(out)
    }

    pub fn list_outbound(&self) -> Result<Vec<OutboundMessage>, ApiError> {
        let mut out = vec![];
        for id in self.db.list_collection_ids("outbound")? {
            out.push(self.db.get_collection_row("outbound", &id)?);
        }
        Ok(out)
    }
}
