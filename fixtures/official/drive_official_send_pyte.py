#!/usr/bin/env python3
"""Add bob as contact on alice and send OFFICIAL_GOLDEN_1 via official TUI + pyte."""
from __future__ import annotations

import os
import time
from pathlib import Path

import pexpect
import pyte

ROOT = Path(__file__).resolve().parents[3]
TUI = ROOT / "glitr-tui-linux"
OFF = Path(__file__).resolve().parent
ALICE = OFF / "alice"
BOB = OFF / "bob"
LOG = OFF / "logs"
XDG = OFF / "xdg-home"
PASSWORD = "official-alice-pass"
PLAINTEXT = "OFFICIAL_GOLDEN_1"
COLS, ROWS = 120, 40


class Term:
    def __init__(self) -> None:
        self.screen = pyte.Screen(COLS, ROWS)
        self.stream = pyte.Stream(self.screen)

    def feed(self, data: bytes) -> None:
        self.stream.feed(data.decode("utf-8", "replace"))

    def text(self) -> str:
        return "\n".join(self.screen.display)

    def find(self, needle: str) -> tuple[int, int] | None:
        for y, line in enumerate(self.screen.display):
            x = line.find(needle)
            if x >= 0:
                return x, y
        return None


def mouse_click(child: pexpect.spawn, x: int, y: int) -> None:
    seq = f"\x1b[<0;{x + 1};{y + 1}M\x1b[<0;{x + 1};{y + 1}m".encode()
    child.send(seq)


def read_into(child: pexpect.spawn, term: Term, seconds: float = 0.5) -> None:
    deadline = time.time() + seconds
    while time.time() < deadline:
        try:
            chunk = child.read_nonblocking(size=100000, timeout=0.2)
            term.feed(chunk)
        except pexpect.TIMEOUT:
            pass
        except pexpect.EOF:
            break


def wait_for(child: pexpect.spawn, term: Term, needles: list[str], timeout: float = 90) -> str:
    deadline = time.time() + timeout
    while time.time() < deadline:
        read_into(child, term, 0.4)
        t = term.text()
        for n in needles:
            if n in t:
                return n
    raise SystemExit(f"timeout waiting for {needles}\n---\n{term.text()[-2000:]}")


def click_label(child: pexpect.spawn, term: Term, label: str) -> bool:
    pos = term.find(label)
    if not pos:
        return False
    mouse_click(child, pos[0], pos[1])
    read_into(child, term, 1.2)
    return True


def main() -> None:
    LOG.mkdir(parents=True, exist_ok=True)
    # Clear prior outbound/contacts from failed runs
    for d in (ALICE / "outbound", ALICE / "contacts", ALICE / "protocol-sessions"):
        if d.exists():
            for p in d.glob("*.json"):
                p.unlink()

    log = open(LOG / "alice-send-pyte.log", "wb")
    env = os.environ.copy()
    env["TERM"] = "xterm-256color"
    env["HOME"] = str(XDG)
    env["XDG_DATA_HOME"] = str(XDG / ".local" / "share")
    env["XDG_CONFIG_HOME"] = str(XDG / ".config")
    env["XDG_STATE_HOME"] = str(XDG / ".local" / "state")

    child = pexpect.spawn(str(TUI), cwd=str(OFF), env=env, timeout=180, dimensions=(ROWS, COLS))
    child.logfile = log
    term = Term()

    wait_for(child, term, ["Profile", "Select a profile", "Glitr", "Mailbox", "last"])
    child.send(str(ALICE).encode() + b"\r")
    hit = wait_for(
        child,
        term,
        ["Encryption password:", "encryption password", "No messages yet", "1:Chats", "Opening mailbox", "Ready"],
    )
    if "password" in hit.lower():
        child.send(PASSWORD.encode() + b"\r")

    # Wait until main chrome is up (Tor banner may flash)
    for _ in range(40):
        read_into(child, term, 1.0)
        t = term.text()
        if "1:Chats" in t and "2:Contacts" in t and "Starting Tor" not in t:
            break
    else:
        print("WARN: continuing despite Tor/chrome wait")

    # Dismiss any search overlay
    for _ in range(3):
        child.send(b"\x1b")
        time.sleep(0.15)
    read_into(child, term, 0.5)

    if not click_label(child, term, "2:Contacts"):
        click_label(child, term, "Contacts")
    read_into(child, term, 1.5)
    print("on contacts?", "No contacts" in term.text() or "Press a" in term.text() or "[Add]" in term.text())

    # Prefer clicking [Add] over key `a` (avoids search focus)
    if not click_label(child, term, "[Add]"):
        if not click_label(child, term, "Add]"):
            child.send(b"a")
            read_into(child, term, 1.5)

    # Wait for add-contact form
    form_hit = None
    for _ in range(20):
        read_into(child, term, 0.5)
        t = term.text()
        if "Display name" in t or "Peer git" in t or "Git remote URL" in t:
            form_hit = t
            break
        if "Search:" in t and "Display name" not in t:
            # Wrong overlay — dismiss and retry Add
            child.send(b"\x1b")
            time.sleep(0.3)
            click_label(child, term, "[Add]") or click_label(child, term, "Add]")
    print("form?", form_hit is not None)
    if form_hit is None:
        print(term.text()[-1200:])
        raise SystemExit("add-contact form not shown")

    # Field order from prior successful run: Display name → Peer git URL → Username → Token
    child.sendcontrol("u")
    child.send(b"OfficialBob")
    child.send(b"\t")
    time.sleep(0.25)
    child.sendcontrol("u")
    child.send(str(BOB).encode())
    child.send(b"\t")
    time.sleep(0.2)
    child.send(b"\t")
    time.sleep(0.2)
    child.send(b"\r")
    read_into(child, term, 10)
    print("listed OfficialBob?", "OfficialBob" in term.text())

    if not click_label(child, term, "OfficialBob"):
        child.send(b"\r")
    read_into(child, term, 1.5)

    # Message compose
    child.send(b"m")
    read_into(child, term, 1.0)
    child.send(b"i")
    read_into(child, term, 0.5)
    child.send(PLAINTEXT.encode())
    child.send(b"\r")
    read_into(child, term, 15)

    child.send(b"\x1b")
    time.sleep(0.2)
    child.sendcontrol("c")
    try:
        child.terminate(force=True)
    except Exception:
        pass
    log.close()

    outs = sorted(p.name for p in (ALICE / "outbound").glob("*.json"))
    cons = sorted(p.name for p in (ALICE / "contacts").glob("*.json"))
    sess = sorted(p.name for p in (ALICE / "protocol-sessions").glob("*.json"))
    print("outbound", outs)
    print("contacts", cons)
    print("sessions", sess)
    if not outs:
        raise SystemExit("no outbound written")


if __name__ == "__main__":
    main()
