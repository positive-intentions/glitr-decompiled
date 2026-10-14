#!/usr/bin/env python3
"""Add bob contact on alice and send OFFICIAL_GOLDEN_1 via official TUI."""
from __future__ import annotations

import os
import time
from pathlib import Path

import pexpect

ROOT = Path(__file__).resolve().parents[3]
TUI = ROOT / "glitr-tui-linux"
OFF = Path(__file__).resolve().parent
ALICE = OFF / "alice"
BOB = OFF / "bob"
LOG = OFF / "logs"
PASSWORD = "official-alice-pass"
PLAINTEXT = "OFFICIAL_GOLDEN_1"


def read_available(child: pexpect.spawn) -> bytes:
    time.sleep(0.4)
    try:
        return child.read_nonblocking(size=100000, timeout=1)
    except pexpect.TIMEOUT:
        return b""
    except pexpect.EOF:
        return b""


def main() -> None:
    LOG.mkdir(exist_ok=True)
    log = open(LOG / "alice-send3.log", "wb")
    env = os.environ.copy()
    env["TERM"] = "xterm-256color"
    child = pexpect.spawn(str(TUI), cwd=str(OFF), env=env, timeout=180, dimensions=(40, 120))
    child.logfile = log

    child.expect_exact([b"Profile [", b"Select a profile", b"Glitr TUI"], timeout=60)
    child.send(str(ALICE).encode() + b"\r")

    for _ in range(50):
        idx = child.expect(
            [
                b"Encryption password:",
                b"encryption password",
                b"Opening mailbox",
                b"No messages yet",
                b"click [actions]",
                b"1:Chats",
                b"Starting Tor",
                pexpect.TIMEOUT,
            ],
            timeout=12,
        )
        if idx in (0, 1):
            child.send(PASSWORD.encode() + b"\r")
        elif idx in (2, 3, 4, 5):
            break

    time.sleep(10)
    for _ in range(4):
        child.send(b"\x1b")
        time.sleep(0.2)

    # Cycle tabs with Tab until Contacts chrome appears
    reached = False
    for i in range(8):
        buf = read_available(child)
        if b"No contacts yet" in buf or b"Press a to add" in buf or b"Add contact" in buf:
            reached = True
            break
        # Also check accumulated log by sending Tab
        child.send(b"\t")
        time.sleep(0.8)
        buf = read_available(child)
        if b"No contacts yet" in buf or b"Press a to add" in buf:
            reached = True
            print("reached contacts after tab", i)
            break
        # Highlight check: look for Contacts without needing empty state
        if b"2:Contacts" in buf and b"Press a" in buf:
            reached = True
            break

    # Fallback: assume after 1 Tab from Chats we are on Contacts
    if not reached:
        print("WARN: Contacts not confirmed; trying a anyway after Esc+Tab")
        for _ in range(4):
            child.send(b"\x1b")
            time.sleep(0.15)
        child.send(b"\t")
        time.sleep(1)

    child.send(b"a")
    time.sleep(1.5)
    buf = read_available(child)
    print("after a, saw Git?", b"Git" in buf, "URL?" , b"URL" in buf)

    # If form opened, fill fields. FormOverlay::add_contact order from strings:
    # Git remote URL (empty = local) / Username (optional) / Access token (optional) / display name
    bob_path = str(BOB)
    # Clear field then type — Ctrl-u clear line if any
    child.sendcontrol("u")
    child.send(bob_path.encode())
    child.send(b"\r")
    time.sleep(0.5)
    child.send(b"\r")
    time.sleep(0.3)
    child.send(b"\r")
    time.sleep(0.3)
    child.send(b"OfficialBob\r")
    time.sleep(6)

    buf = read_available(child)
    print("after form, OfficialBob on screen?", b"OfficialBob" in buf)

    # Open contact details / chat
    child.send(b"\r")
    time.sleep(1)
    child.send(b"m")
    time.sleep(1.5)
    child.send(b"i")
    time.sleep(0.5)
    child.send(PLAINTEXT.encode())
    child.send(b"\r")
    time.sleep(10)

    child.send(b"\x1b")
    time.sleep(0.3)
    child.sendcontrol("c")
    try:
        child.terminate(force=True)
    except Exception:
        pass
    log.close()

    outs = [p.name for p in (ALICE / "outbound").glob("*.json")]
    cons = [p.name for p in (ALICE / "contacts").glob("*.json")]
    print("outbound", outs)
    print("contacts", cons)


if __name__ == "__main__":
    main()
