//! ApiHandle — product shell entry.
//! EVIDENCE: glitr_client::handle::ApiHandle::{new,dispatch,start_git_worker,memory}

use std::path::Path;

use api_core::MailboxApi;

pub struct ApiHandle {
    pub api: MailboxApi,
}

impl ApiHandle {
    pub fn open(workdir: impl AsRef<Path>, password: &str) -> anyhow::Result<Self> {
        Ok(Self {
            api: MailboxApi::connect(workdir, password)?,
        })
    }
}
