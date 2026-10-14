# Source: https://glitr.io/docs/technical/ui-ux/
# HTTP: 200

Technical
UI/UX

# UI/UX

Research, not an auditThese pages are research and development and may not match the live app. Do not treat this as an audit. Shared for testing and demo only. Please use responsibly. The messenger starts as screens: connect, chats, a thread, contacts, calls, a profile. Those screens live in two crates. whatsup is the product shell — routes, PWA, and platform entry points. whatsup-ui is the chrome those routes mount: lists, threads, ticks, pairing panels, and a terminal frontend.
This page is the UI layer. Session, poll, and send live in glitr-client and are covered on Network. How the shell sits on the rest of the stack is the Architecture product map.
The UI crates are closed-source. Galleries are the public surface.

## whatsup​

whatsup is the Glitr client: a Dioxus 0.6 app for the web PWA, desktop, mobile, and a text TUI. It owns the router and the platform binaries. The chrome is imported from whatsup-ui.
The published demo on GitHub Pages is a UI shell with mock data — hardcoded contacts, threads, and replies so the screens can be pressed without a mailbox.
Open the Glitr demo
The product app on this site is the same shell wired to glitr-client. Use Try messaging when you want two tabs that actually send.
Platform
What you get
Web
dx serve default. WASM plus origin-private storage for a real mailbox. The Onion chip is hidden; this build does not run Arti.
Desktop
Native git on the filesystem. Live and calls depend on the webview. After login the shell starts Tor: onion pairing, a status line, and SOCKS for git.
Mobile
Android and iOS nightlies. Cargo can enable Tor; it is not a claimed product path.
TUI
whatsup-tui: text poll and send, onion paste (o), Tor status, and Ctrl-t to retry bootstrap. Voice and video stay on the graphical app.
Native chrome shows Onion on contact add and Tor status on connect and settings. whatsup-ui only draws those controls. whatsup is what starts Tor after login. How that session is wired is on Network.
Cargo features are exclusive — one frontend at a time. There is no Tailwind step in this crate: the stylesheet ships in whatsup-ui.

## whatsup-ui​

The chrome outgrew one app crate. Lists, threads, ticks, pairing, and the terminal frontend live in whatsup-ui so every gallery in the stack can import the same components.
One domain model, two frontends:
gui — Dioxus components for web, desktop, and mobile.
tui — ratatui draw functions for the terminal.
The components are router-agnostic. They describe where they want to go with a NavTarget. The host maps that onto its own routes. A mock feature seeds contacts, chats, call history, and canned replies so a story can render without a mailbox.
Web UI testing usually means Storybook. There is no Storybook-for-Rust that fits this stack (thread), so the crate ships its own gallery: /demo/:kind/:group/:name, knobs, docs tabs, and a browser terminal that can show a TUI. It is an internal debugging tool that happens to be public.
Open the component gallery

### Page stories​

Atoms, molecules, and knobs stay on the gallery home. These are the screens that matter for the messenger.
Story
Live
Chat thread
gui/pages/chatthread
Chats
gui/pages/chats
Contacts
gui/pages/contacts
Contact details
gui/pages/contact-details
Calls
gui/pages/calls
Active call
gui/pages/activecall
Profile
gui/pages/profile
Chat thread (TUI)
tui/pages/chatthread
The same gallery chrome is reused by the crypto, git, db, api, webrtc, and tor galleries. Those stories are on the pages that follow.
Next: Cryptography — Signal, ML-KEM, and PQXDH as libraries you can press.