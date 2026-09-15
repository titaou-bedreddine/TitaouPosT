# TitaouPOS Licensing v0.6.0 — Signed Licenses, Read-Only Enforcement & Revocation

This replaces the fake activation of v0.5.x (any code ≥12 chars activated)
with a real system: **Ed25519-minisign signed licenses bound to each PC**,
**no license = READ-ONLY**, and **GitHub-registry revocation**.

## The moving parts

| Piece | Where | Role |
|---|---|---|
| Master keypair | `%APPDATA%\TitaouPosT\license_master.key` (dev PC only) | SECRET key signs every license; the PUBLIC key is compiled into the POS |
| **License Generator** (standalone app) | `license-generator/` in this repo | The developer's tool: paste a client request code → sign Full/Trial → copy key / download .lic / publish online / revoke |
| Registry repo | [titaou-bedreddine/TitaouPosT-licenses](https://github.com/titaou-bedreddine/TitaouPosT-licenses) | `licenses/<HWID>.json` = online activation; `revoked.json` = revocation list |
| POS gate | `src-tauri/src/services/license_service.rs` + `lib.rs` invoke wrapper + LAN server dispatch | Every business mutation is refused (`APP_READ_ONLY`) while this terminal has no active license |

## The flow end-to-end

1. **Client installs the POS.** The first-run setup wizard (now 4 steps)
   ends with the **Activation Request Code** — one line combining
   `HWID + shop + owner`, with a Copy button and a QR code
   (`TIT-REQ|v=1|hw=HW-…|shop=…|owner=…`).
2. **The developer opens the License Generator**, pastes the request code
   (or scans the QR), picks **Full (lifetime)** or **Trial (N days)**, and
   clicks **Sign License**.
3. **Delivery — any of:**
   - **.lic file** (preferred): client drags & drops it into
     Settings → Activation.
   - **Serial key**: client pastes it (or uses the **Paste & Activate**
     button) in Activation.
   - **Publish Online** (generator button): writes
     `licenses/<HWID>.json` to the registry repo; the client clicks
     **Activate This PC Online** — if nothing is published yet, the POS
     automatically falls back to sending the activation request to the
     developer over Telegram.
   - **Telegram approval card** (unchanged from v0.5.34): the client's
     Request button pops the card on the developer's POS — Full /
     Trial 14d / Reject.
4. **The POS verifies** the Ed25519 signature against the embedded public
   key, checks the **HWID matches this machine**, and that a trial hasn't
   expired. Only then activation persists. The registry JSON is only
   transport — **the signature is the trust boundary** (a client with the
   bot token or DB access gains nothing).

## No license = READ-ONLY

- Gated commands (refused with `APP_READ_ONLY: …`): sales, products,
  purchases, expenses, cash register, customers/suppliers, employees,
  users, scale sync, factory reset / restore.
- Open on purpose: all reads, settings (setup must work pre-license),
  the whole activation surface, support, updates.
- Enforcement lives in THREE places so nothing bypasses it:
  1. the Tauri invoke wrapper (`lib.rs`) — the local UI,
  2. the LAN server `dispatch()` — connected client terminals,
  3. boot revalidation — the stored license key is re-verified at every
     startup; a DB-edited `app_license_status` without a valid signed key
     is wiped back to `none`.
- The `app_license_*` settings keys are only writable through
  `license_service` — generic `set_setting` / `set_multiple_settings`
  (UI, LAN API, restored settings sidecars) silently drop them.
- An amber **READ-ONLY banner** sits at the top of the app with an
  Activate button; it lifts immediately on activation (no restart).

Legacy note: pre-signed-era machines that activated with a manual code
(`app_license_status = "activated"`) are grandfathered as FULL.

## Revocation (the serial that must never work again)

- In the License Generator: paste the HWID → **Revoke**. The HWID is added
  to `revoked.json` in the registry repo.
- Every POS checks `revoked.json`:
  - at activation (a revoked machine cannot re-activate),
  - every 30 minutes in the background (offline machines flip at their
    next successful check).
- A revoked terminal goes READ-ONLY within ~30 minutes. The status is
  one-way: only issuing a NEW signed license (after **Un-revoke**)
  reactivates the machine.
- Because every license is bound to the machine HWID, a serial moved to
  another PC fails verification anyway — revocation kills the original
  machine too.

## License Generator — how to run/build

```
cd license-generator/src-tauri
cargo tauri build        # release exe + installers in target/release/bundle
cargo tauri dev          # dev run
```

- The master key is shared with the in-app License Studio
  (`%APPDATA%\TitaouPosT\license_master.key`) — generate it ONCE, on the
  developer machine only, and back it up. Losing it = no new licenses.
- Publishing/revoking uses the GitHub token from `gh auth login` (or
  `GITHUB_TOKEN`).

## POS build

Unchanged: `npx tauri build` at the repo root. v0.6.0 embeds the same
minisign public key as v0.5.34 — existing signed licenses keep working.
