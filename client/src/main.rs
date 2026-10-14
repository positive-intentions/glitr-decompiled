//! Compatible Glitr git-mailbox CLI (reconstruction).
//! Evidence boundary: docs + binaries under target/ only.

use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use api_core::{SignalingPayload, DEFAULT_STUN};
use clap::{Parser, Subcommand};
use glitr_client::ApiHandle;

#[derive(Parser, Debug)]
#[command(name = "glitr-compat", about = "Glitr investigation compatible mailbox client")]
struct Cli {
    #[arg(long, env = "GLITR_PASSWORD", default_value = "investigation")]
    password: String,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Bootstrap a mailbox workdir with schema v10 stamps
    Init {
        #[arg(long)]
        workdir: PathBuf,
        #[arg(long, default_value = "Investigator")]
        display_name: String,
        #[arg(long, default_value = "investigator")]
        handle: String,
    },
    /// Show local profile-public.json
    Profile {
        #[arg(long)]
        workdir: PathBuf,
    },
    /// Add a git-URL contact (loads peer profile-public when git_url is a local path)
    AddContact {
        #[arg(long)]
        workdir: PathBuf,
        #[arg(long, default_value = "")]
        peer_profile_id: String,
        #[arg(long)]
        git_url: String,
        #[arg(long)]
        display_name: String,
        #[arg(long)]
        peer_public_key_file: Option<PathBuf>,
        #[arg(long)]
        token: Option<String>,
    },
    /// List contacts
    Contacts {
        #[arg(long)]
        workdir: PathBuf,
    },
    /// Send a text message to a contact id
    Send {
        #[arg(long)]
        workdir: PathBuf,
        #[arg(long)]
        contact_id: String,
        #[arg(long)]
        body: String,
    },
    /// Poll/ingest outbound from a peer workdir into local inbox
    Poll {
        #[arg(long)]
        workdir: PathBuf,
        #[arg(long)]
        peer_workdir: PathBuf,
        #[arg(long)]
        contact_id: String,
    },
    /// List inbox
    Inbox {
        #[arg(long)]
        workdir: PathBuf,
    },
    /// List outbound
    Outbound {
        #[arg(long)]
        workdir: PathBuf,
    },
    /// Export peer-visible public key (base64 DER) for pairing
    ExportPubkey {
        #[arg(long)]
        workdir: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    /// Post a WebRTC signaling payload (offer/answer/candidate/chat)
    Signal {
        #[arg(long)]
        workdir: PathBuf,
        #[arg(long)]
        recipient_id: String,
        #[arg(long)]
        role: String,
        #[arg(long)]
        kind: String,
        #[arg(long)]
        sdp: Option<String>,
        #[arg(long)]
        chat: Option<String>,
    },
    /// List signaling rows for recipient
    Signals {
        #[arg(long)]
        workdir: PathBuf,
        #[arg(long)]
        recipient_id: String,
    },
    /// Poll peer signaling into local mailbox
    PollSignals {
        #[arg(long)]
        workdir: PathBuf,
        #[arg(long)]
        peer_workdir: PathBuf,
    },
    /// Serve official glitr-web static assets + a tiny recon API for local testing
    Serve {
        #[arg(long, default_value = "8787")]
        port: u16,
        #[arg(long)]
        web_root: PathBuf,
        #[arg(long)]
        workdir: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Init {
            workdir,
            display_name,
            handle,
        } => {
            let h = ApiHandle::open(&workdir, &cli.password)?;
            let pubp = h.api.setup_profile(&display_name, &handle)?;
            println!(
                "initialized {} profile id={} schema=messaging/{} stun={}",
                workdir.display(),
                pubp.id,
                messaging_schema::SCHEMA_VERSION,
                DEFAULT_STUN
            );
        }
        Cmd::Profile { workdir } => {
            let h = ApiHandle::open(&workdir, &cli.password)?;
            let p = h.api.profile_public()?;
            println!("{}", serde_json::to_string_pretty(&p)?);
        }
        Cmd::AddContact {
            workdir,
            peer_profile_id,
            git_url,
            display_name,
            peer_public_key_file,
            token,
        } => {
            let h = ApiHandle::open(&workdir, &cli.password)?;
            let pubkey = match peer_public_key_file {
                Some(p) => Some(std::fs::read_to_string(p)?),
                None => None,
            };
            let c = h.api.add_contact(
                &peer_profile_id,
                &git_url,
                &display_name,
                pubkey.as_deref(),
                token.as_deref(),
            )?;
            println!(
                "contact id={} peer={} webrtc={} stun={}",
                c.id,
                c.peer_profile_id,
                c.webrtc_enabled.as_deref().unwrap_or(""),
                c.stun_url.as_deref().unwrap_or("")
            );
        }
        Cmd::Contacts { workdir } => {
            let h = ApiHandle::open(&workdir, &cli.password)?;
            for c in h.api.list_contacts()? {
                println!(
                    "{}  peer={}  name={}  git={}",
                    c.id, c.peer_profile_id, c.display_name, c.git_url
                );
            }
        }
        Cmd::Send {
            workdir,
            contact_id,
            body,
        } => {
            let h = ApiHandle::open(&workdir, &cli.password)?;
            let o = h.api.send_text(&contact_id, &body)?;
            println!("outbound id={} recipient={}", o.id, o.recipient_id);
        }
        Cmd::Poll {
            workdir,
            peer_workdir,
            contact_id,
        } => {
            let h = ApiHandle::open(&workdir, &cli.password)?;
            let msgs = h
                .api
                .ingest_peer_outbound_dir(&peer_workdir, &contact_id)?;
            println!("ingested {} messages", msgs.len());
            for m in msgs {
                println!("  [{}] {}", m.id, m.body);
            }
        }
        Cmd::Inbox { workdir } => {
            let h = ApiHandle::open(&workdir, &cli.password)?;
            for m in h.api.list_inbox()? {
                println!("{} <- {}: {}", m.id, m.sender_id, m.body);
            }
        }
        Cmd::Outbound { workdir } => {
            let h = ApiHandle::open(&workdir, &cli.password)?;
            for m in h.api.list_outbound()? {
                println!(
                    "{} -> {} ct_len={}",
                    m.id,
                    m.recipient_id,
                    m.ciphertext.len()
                );
            }
        }
        Cmd::ExportPubkey { workdir, out } => {
            let h = ApiHandle::open(&workdir, &cli.password)?;
            let p = h.api.profile_public().context("profile")?;
            std::fs::write(&out, p.public_key.as_bytes())?;
            println!("wrote {} (profile {})", out.display(), p.id);
            if p.protocol_bundles.is_none() {
                bail!("unexpected: no protocol_bundles field");
            }
        }
        Cmd::Signal {
            workdir,
            recipient_id,
            role,
            kind,
            sdp,
            chat,
        } => {
            let h = ApiHandle::open(&workdir, &cli.password)?;
            let frame = chat.map(|body| api_core::ChatFrame::Chat {
                body,
                outbound_message_id: None,
            });
            let payload = SignalingPayload {
                kind,
                sdp,
                candidate: None,
                frame,
            };
            let s = h.api.post_signaling(&recipient_id, &role, &payload)?;
            println!("signaling id={} role={:?}", s.id, s.role);
        }
        Cmd::Signals {
            workdir,
            recipient_id,
        } => {
            let h = ApiHandle::open(&workdir, &cli.password)?;
            for s in h.api.list_signaling_for(&recipient_id)? {
                println!(
                    "{} from={} role={:?} ct={}",
                    s.id,
                    s.sender_id,
                    s.role,
                    s.ciphertext
                );
            }
        }
        Cmd::PollSignals {
            workdir,
            peer_workdir,
        } => {
            let h = ApiHandle::open(&workdir, &cli.password)?;
            let rows = h.api.poll_peer_signaling(&peer_workdir)?;
            println!("polled {} signaling rows", rows.len());
        }
        Cmd::Serve {
            port,
            web_root,
            workdir,
        } => {
            serve_lab(port, &web_root, &workdir, &cli.password)?;
        }
    }
    Ok(())
}

fn serve_lab(port: u16, web_root: &Path, workdir: &Path, password: &str) -> Result<()> {
    use std::io::Read;
    use std::net::TcpListener;

    let listener = TcpListener::bind(("127.0.0.1", port))?;
    println!("glitr-compat lab listening on http://127.0.0.1:{port}");
    println!("  official web UI:  http://127.0.0.1:{port}/");
    println!("  webrtc lab:       http://127.0.0.1:{port}/lab/webrtc.html");
    println!("  recon API:        http://127.0.0.1:{port}/api/profile");
    println!("  mailbox workdir:  {}", workdir.display());

    let lab_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../fixtures/test-lab");

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buf = [0u8; 8192];
        let n = stream.read(&mut buf)?;
        let req = String::from_utf8_lossy(&buf[..n]);
        let mut lines = req.lines();
        let first = lines.next().unwrap_or("");
        let mut parts = first.split_whitespace();
        let method = parts.next().unwrap_or("GET");
        let path = parts.next().unwrap_or("/");

        if method == "GET" && path == "/api/profile" {
            let h = ApiHandle::open(workdir, password)?;
            let p = h.api.profile_public()?;
            let body = serde_json::to_vec_pretty(&p)?;
            write_response(&mut stream, 200, "application/json", &body)?;
            continue;
        }
        if method == "GET" && path == "/api/contacts" {
            let h = ApiHandle::open(workdir, password)?;
            let body = serde_json::to_vec_pretty(&h.api.list_contacts()?)?;
            write_response(&mut stream, 200, "application/json", &body)?;
            continue;
        }
        if method == "GET" && path == "/api/inbox" {
            let h = ApiHandle::open(workdir, password)?;
            let body = serde_json::to_vec_pretty(&h.api.list_inbox()?)?;
            write_response(&mut stream, 200, "application/json", &body)?;
            continue;
        }
        if method == "GET" && path.starts_with("/api/signals?") {
            let recipient = path
                .split('?')
                .nth(1)
                .unwrap_or("")
                .split('&')
                .find_map(|kv| kv.strip_prefix("recipient="))
                .unwrap_or("");
            let h = ApiHandle::open(workdir, password)?;
            let body = serde_json::to_vec_pretty(&h.api.list_signaling_for(recipient)?)?;
            write_response(&mut stream, 200, "application/json", &body)?;
            continue;
        }

        // Static: /lab/* from fixtures/test-lab, else glitr-web root
        let rel = if let Some(rest) = path.strip_prefix("/lab/") {
            lab_dir.join(rest)
        } else if path == "/" || path.is_empty() {
            web_root.join("index.html")
        } else {
            web_root.join(path.trim_start_matches('/'))
        };
        match std::fs::read(&rel) {
            Ok(bytes) => {
                let ctype = match rel.extension().and_then(|e| e.to_str()) {
                    Some("html") => "text/html; charset=utf-8",
                    Some("js") => "application/javascript",
                    Some("css") => "text/css",
                    Some("wasm") => "application/wasm",
                    Some("json") => "application/json",
                    Some("svg") => "image/svg+xml",
                    Some("png") => "image/png",
                    _ => "application/octet-stream",
                };
                write_response(&mut stream, 200, ctype, &bytes)?;
            }
            Err(_) => {
                write_response(&mut stream, 404, "text/plain", b"not found")?;
            }
        }
    }
    Ok(())
}

fn write_response(
    stream: &mut impl std::io::Write,
    status: u16,
    ctype: &str,
    body: &[u8],
) -> Result<()> {
    let reason = match status {
        200 => "OK",
        404 => "Not Found",
        _ => "Error",
    };
    let header = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {ctype}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(header.as_bytes())?;
    stream.write_all(body)?;
    Ok(())
}
