//! git-core façade over a workdir.
//!
//! EVIDENCE: docs Persistence (GitRepo trait; native gitoxide / browser isomorphic-git).
//! Reconstruction uses the system `git` CLI so we stay dependency-light without claiming
//! gitoxide wire parity. Embedded paths: git/git-core/src/{native,sync,types}.rs

use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, thiserror::Error)]
pub enum GitError {
    #[error("git: {0}")]
    Msg(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
pub struct GitRepo {
    pub root: PathBuf,
}

impl GitRepo {
    pub fn init(path: impl AsRef<Path>) -> Result<Self, GitError> {
        let root = path.as_ref().to_path_buf();
        std::fs::create_dir_all(&root)?;
        run_git(&root, &["init", "-b", "main"])?;
        // EVIDENCE (binary): Sync from database //main/master — product uses main.
        Ok(Self { root })
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self, GitError> {
        let root = path.as_ref().to_path_buf();
        if !root.join(".git").exists() {
            return Err(GitError::Msg(format!("not a git repo: {}", root.display())));
        }
        Ok(Self { root })
    }

    pub fn write_file(&self, rel: &str, bytes: &[u8]) -> Result<(), GitError> {
        let path = self.root.join(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, bytes)?;
        Ok(())
    }

    pub fn read_file(&self, rel: &str) -> Result<Vec<u8>, GitError> {
        Ok(std::fs::read(self.root.join(rel))?)
    }

    pub fn exists(&self, rel: &str) -> bool {
        self.root.join(rel).exists()
    }

    pub fn list_dir(&self, rel: &str) -> Result<Vec<String>, GitError> {
        let dir = self.root.join(rel);
        if !dir.exists() {
            return Ok(vec![]);
        }
        let mut out = vec![];
        for ent in std::fs::read_dir(dir)? {
            let ent = ent?;
            if ent.file_type()?.is_file() {
                if let Some(name) = ent.file_name().to_str() {
                    out.push(name.to_string());
                }
            }
        }
        out.sort();
        Ok(out)
    }

    pub fn commit_all(&self, message: &str) -> Result<(), GitError> {
        run_git(&self.root, &["add", "-A"])?;
        // Allow empty for first stamp-only commits
        let status = Command::new("git")
            .current_dir(&self.root)
            .args(["diff", "--cached", "--quiet"])
            .status()?;
        if status.success() {
            return Ok(()); // nothing staged
        }
        run_git(
            &self.root,
            &[
                "-c",
                "user.email=glitr-investigation@local",
                "-c",
                "user.name=glitr-investigation",
                "commit",
                "-m",
                message,
            ],
        )?;
        Ok(())
    }

    pub fn add_remote(&self, name: &str, url: &str) -> Result<(), GitError> {
        let _ = run_git(&self.root, &["remote", "remove", name]);
        run_git(&self.root, &["remote", "add", name, url])
    }

    pub fn push(&self, remote: &str, branch: &str) -> Result<(), GitError> {
        run_git(&self.root, &["push", "-u", remote, branch])
    }

    pub fn fetch(&self, remote: &str) -> Result<(), GitError> {
        run_git(&self.root, &["fetch", remote])
    }

    pub fn pull_ff(&self, remote: &str, branch: &str) -> Result<(), GitError> {
        run_git(&self.root, &["pull", "--ff-only", remote, branch])
    }
}

fn run_git(cwd: &Path, args: &[&str]) -> Result<(), GitError> {
    let out = Command::new("git").current_dir(cwd).args(args).output()?;
    if out.status.success() {
        Ok(())
    } else {
        Err(GitError::Msg(format!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr)
        )))
    }
}
