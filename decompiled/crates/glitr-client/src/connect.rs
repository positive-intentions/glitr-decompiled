//! Saved connect config.
//! EVIDENCE (binary): SavedConnect with fields url, token, encryption_password, cors_proxy, remotes, ...

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SavedConnect {
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default)]
    pub encryption_password: Option<String>,
    #[serde(default)]
    pub cors_proxy: Option<String>,
    #[serde(default)]
    pub local_only: Option<bool>,
    #[serde(default)]
    pub workdir: Option<String>,
    #[serde(default)]
    pub remotes: Vec<SavedRemote>,
    #[serde(default)]
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedRemote {
    pub name: String,
    pub url: String,
    #[serde(default)]
    pub token: Option<String>,
}
