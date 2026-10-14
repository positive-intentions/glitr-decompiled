#!/usr/bin/env python3
"""Bootstrap official mailbox with isolated XDG so global profiles cannot pollute."""
from __future__ import annotations

import os
import shutil
import time
from pathlib import Path

import pexpect

ROOT = Path(__file__).resolve().parents[3]
TUI = ROOT / "glitr-tui-linux"
OFF = Path(__file__).resolve().parent
LOG = OFF / "logs"
XDG = OFF / "xdg-home"


def isolated_env() -> dict:
    env = os.environ.copy()
    env["TERM"] = "xterm-256color"
    env["HOME"] = str(XDG)
    env["XDG_DATA_HOME"] = str(XDG / ".local" / "share")
    env["XDG_CONFIG_HOME"] = str(XDG / ".config")
    env["XDG_STATE_HOME"] = str(XDG / ".local" / "state")
    (XDG / ".local" / "share").mkdir(parents=True, exist_ok=True)
    (XDG / ".config").mkdir(parents=True, exist_ok=True)
    return env


def drive_bootstrap(workdir: Path, password: str, display_name: str, log_name: str) -> None:
    if workdir.exists():
        shutil.rmtree(workdir)
    workdir.mkdir(parents=True, exist_ok=True)
    logfile = open(LOG / log_name, "wb")
    env = isolated_env()

    child = pexpect.spawn(str(TUI), cwd=str(OFF), env=env, timeout=90, dimensions=(40, 120))
    child.logfile = logfile

    child.expect_exact([b"Profile [", b"Select a profile", b"Glitr TUI", b"No profile"], timeout=60)
    child.send(str(workdir).encode() + b"\r")

    answered_pw = False
    answered_git = False
    answered_user = False
    answered_token = False
    answered_name = False
    deadline = time.time() + 120

    while time.time() < deadline:
        idx = child.expect(
            [
                b"Encryption password:",
                b"encryption password",
                b"password required",
                b"Git remote URL",
                b"Git username",
                b"Git token",
                b"Display name",
                b"Opening mailbox",
                b"Ready",
                b"Loading chats",
                b"1:Chats",
                b"Chats",
                b"No messages yet",
                b"Mailbox updated",
                b"click [actions]",
                b"New mailbox folder",
                pexpect.TIMEOUT,
                pexpect.EOF,
            ],
            timeout=20,
        )
        if idx == 15:
            child.send(str(workdir).encode() + b"\r")
            continue
        if idx in (0, 1, 2) and not answered_pw:
            child.send(password.encode() + b"\r")
            answered_pw = True
        elif idx == 3 and not answered_git:
            child.send(b"\r")
            answered_git = True
        elif idx == 4 and not answered_user:
            child.send(b"\r")
            answered_user = True
        elif idx == 5 and not answered_token:
            child.send(b"\r")
            answered_token = True
        elif idx == 6 and not answered_name:
            child.send(display_name.encode() + b"\r")
            answered_name = True
        elif idx in (7, 8, 9, 10, 11, 12, 13, 14):
            # Wait for collections + sync before killing
            time.sleep(8)
            child.sendcontrol("c")
            try:
                child.terminate(force=True)
            except Exception:
                pass
            logfile.close()
            return
        elif idx == 17:
            logfile.close()
            raise SystemExit(f"EOF: {log_name}")

    child.terminate(force=True)
    logfile.close()
    raise SystemExit(f"timeout: {log_name}")


def main() -> None:
    LOG.mkdir(parents=True, exist_ok=True)
    # Wipe XDG so profile index + API cache cannot reuse stale peer ids
    if XDG.exists():
        shutil.rmtree(XDG)
    XDG.mkdir(parents=True)
    drive_bootstrap(OFF / "alice", "official-alice-pass", "OfficialAlice", "alice-boot.log")
    print("alice", sorted(p.name for p in (OFF / "alice").iterdir() if p.is_file()))
    # Clear connect.json display_name bleed between boots
    connect = XDG / ".local/share/positive-intentions/glitr/session/glitr_connect.json"
    if connect.exists():
        connect.unlink()
    drive_bootstrap(OFF / "bob", "official-bob-pass", "OfficialBob", "bob-boot.log")
    print("bob", sorted(p.name for p in (OFF / "bob").iterdir() if p.is_file()))


if __name__ == "__main__":
    main()
