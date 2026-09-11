# RustDesk Remote Support — Integration Plan

Goal: the shop owner can remotely help any TitaouPOS PC with **one setup**
and a **one-click Help button** on the client, with auto-permission agreed
verbally by the client at installation time.

## Architecture

```
Client PC (TitaouPOS)                     Owner (any PC/phone)
┌─────────────────────────┐               ┌──────────────────────┐
│ [Help] button           │   Telegram    │ RustDesk client      │
│  └→ request_support()   │ ────────────► │ (+ Telegram message) │
│      RustDesk --get-id  │               │  connect ID+password │
│      ID + shop password │               └──────────────────────┘
└─────────────────────────┘
```

The app never opens a browser and never exposes the credentials anywhere
except the owner's own Telegram chat.

## Phase 0 — SHIPPED (v0.5.29)
- Sidebar **Help** button next to the language toggles.
- `request_support` Rust command: auto-detects RustDesk (Program Files,
  x86, %LOCALAPPDATA%) or uses the `rustdesk_path` setting; reads the
  RustDesk ID (`rustdesk --get-id`, hidden window); attaches the shared
  `rustdesk_support_password` setting; sends a localized Telegram alert
  (PC name + ID + password) directly to the owner's bot chat.
- Settings → Network → **Remote Support (RustDesk)** card: optional
  executable path override (per-PC) and the shared access password
  (shop-wide, propagates to every client).
- Clear errors: RustDesk missing / Telegram not configured.

## Phase 1 — One-Setup bundling (single installer)
1. Download the official portable `rustdesk.exe` (hosted release) once and
   place it at `src-tauri/resources/rustdesk/rustdesk.exe`.
2. `tauri.conf.json → bundle.resources`: include
   `"resources/rustdesk/rustdesk.exe"` — it lands next to the app and is
   removed with the app on uninstall.
3. On first run (FirstSetupWizard step or Network settings card), the app:
   - copies the binary to `%LOCALAPPDATA%\TitaouPOS\support\rustdesk.exe`,
   - runs `rustdesk.exe --install-service` (one elevation prompt) so the
     unattended service starts with Windows,
   - sets the shop-wide unattended password:
     `rustdesk.exe --password <rustdesk_support_password>`.
4. Consent: the setup wizard shows a checkbox —
   "Allow the shop owner to connect to this PC for remote support
   (verbal agreement)" — default OFF; enabling it performs step 3.
   The consent flag is stored as the `rustdesk_consent` setting.

## Phase 2 — Owner-side UX
1. The Telegram message already carries ID + password.
2. Optional: a "Support Requests" card on the server Dashboard — clients
   POST `/api/v1/support` (device-token auth) and the server broadcasts a
   `support_requested` event; the owner's UI shows a toast with a
   **Connect** button that shells `rustdesk.exe --connect <id>
   --password <pw>`.
3. Optional: ring/notification sound on incoming requests.

## Phase 3 — Hardening
- Rotate the support password from Settings (regenerates on all clients
  via the shop-wide settings sync + `rustdesk --password`).
- Revoke consent per PC (stops the service, clears the password).
- Whitelist the direct RustDesk relay in the firewall alongside the POS
  port (the "Allow through Windows Firewall" flow already exists).

## Security & licensing notes
- **Consent**: unattended access is configured ONLY after the checkbox
  above is enabled — the verbal agreement you take at installation is the
  consent, and it is revocable in one click.
- **Credentials** live in the shop database (settings) and travel only to
  the owner's Telegram chat over HTTPS.
- **Licensing**: RustDesk is AGPL-3.0 — bundling the official unmodified
  binary is permitted; keep the RustDesk version + source link in the
  About tab and ship the corresponding source offer with the installer.
- **Audit**: every support connection attempt is visible in the RustDesk
  client log on the client PC.
