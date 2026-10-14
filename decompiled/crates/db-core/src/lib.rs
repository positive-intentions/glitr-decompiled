//! db-core file store over git workdir with optional password sealing.
//!
//! EVIDENCE: docs Persistence + Data structures; embedded db/db-core/src/*.rs
//! Official uses GraphQL; reconstruction exposes typed file helpers matching SDL paths.

use std::path::Path;

use crypto_core::{seal_password, unseal_password, PasswordEnvelope};
use git_core::GitRepo;
use messaging_schema::{
    collection_dirs, is_sealed_path, SchemaStamp, APP_NAME, SCHEMA_VERSION,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("db: {0}")]
    Msg(String),
    #[error(transparent)]
    Git(#[from] git_core::GitError),
    #[error(transparent)]
    Crypto(#[from] crypto_core::CryptoError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

pub struct MailboxDb {
    pub git: GitRepo,
    password: Option<String>,
}

impl MailboxDb {
    pub fn bootstrap(path: impl AsRef<Path>, password: Option<&str>) -> Result<Self, DbError> {
        let git = if path.as_ref().join(".git").exists() {
            GitRepo::open(path)?
        } else {
            GitRepo::init(path)?
        };
        let db = Self {
            git,
            password: password.map(|s| s.to_string()),
        };
        db.write_schema_stamps()?;
        for dir in collection_dirs() {
            std::fs::create_dir_all(db.git.root.join(dir))
                .map_err(|e| DbError::Msg(e.to_string()))?;
        }
        db.git.commit_all("bootstrap messaging schema")?;
        Ok(db)
    }

    pub fn open(path: impl AsRef<Path>, password: Option<&str>) -> Result<Self, DbError> {
        let git = GitRepo::open(path)?;
        Ok(Self {
            git,
            password: password.map(|s| s.to_string()),
        })
    }

    fn write_schema_stamps(&self) -> Result<(), DbError> {
        // EVIDENCE: schema.json + schemaVersion.json stay plaintext.
        let stamp = SchemaStamp::messaging_v10();
        self.git.write_file(
            "schemaVersion.json",
            serde_json::to_vec_pretty(&stamp)?.as_slice(),
        )?;
        // Minimal schema.json marker (official embeds full GraphQL SDL — UNKNOWN exact file body).
        let schema = serde_json::json!({
            "app": APP_NAME,
            "version": SCHEMA_VERSION,
            "source": "reconstruction",
            "note": "Full GraphQL SDL lives in evidence/messaging.v10.graphql"
        });
        self.git
            .write_file("schema.json", serde_json::to_vec_pretty(&schema)?.as_slice())?;
        Ok(())
    }

    pub fn put_json<T: Serialize>(&self, rel: &str, value: &T) -> Result<(), DbError> {
        let bytes = serde_json::to_vec_pretty(value)?;
        if is_sealed_path(rel) {
            let pw = self
                .password
                .as_deref()
                .ok_or_else(|| DbError::Msg("password required for sealed write".into()))?;
            let env = seal_password(pw, &bytes)?;
            self.git
                .write_file(rel, serde_json::to_vec_pretty(&env)?.as_slice())?;
        } else {
            self.git.write_file(rel, &bytes)?;
        }
        Ok(())
    }

    pub fn get_json<T: DeserializeOwned>(&self, rel: &str) -> Result<T, DbError> {
        let raw = self.git.read_file(rel)?;
        if is_sealed_path(rel) {
            let pw = self
                .password
                .as_deref()
                .ok_or_else(|| DbError::Msg("password required for sealed read".into()))?;
            // Try envelope first; fall back to plaintext for migration/debug.
            if let Ok(env) = serde_json::from_slice::<PasswordEnvelope>(&raw) {
                let pt = unseal_password(pw, &env)?;
                return Ok(serde_json::from_slice(&pt)?);
            }
        }
        Ok(serde_json::from_slice(&raw)?)
    }

    pub fn put_collection_row<T: Serialize>(
        &self,
        collection: &str,
        id: &str,
        value: &T,
    ) -> Result<(), DbError> {
        self.put_json(&format!("{collection}/{id}.json"), value)
    }

    pub fn get_collection_row<T: DeserializeOwned>(
        &self,
        collection: &str,
        id: &str,
    ) -> Result<T, DbError> {
        self.get_json(&format!("{collection}/{id}.json"))
    }

    pub fn list_collection_ids(&self, collection: &str) -> Result<Vec<String>, DbError> {
        let mut ids = vec![];
        for name in self.git.list_dir(collection)? {
            if let Some(id) = name.strip_suffix(".json") {
                ids.push(id.to_string());
            }
        }
        Ok(ids)
    }

    pub fn read_raw_value(&self, rel: &str) -> Result<Value, DbError> {
        let raw = self.git.read_file(rel)?;
        Ok(serde_json::from_slice(&raw)?)
    }

    pub fn commit(&self, msg: &str) -> Result<(), DbError> {
        self.git.commit_all(msg)?;
        Ok(())
    }
}
