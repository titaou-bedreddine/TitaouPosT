//! Remote support via RustDesk (v0.5.29).
//!
//! A client-PC user clicks the sidebar Help button: this service locates the
//! local RustDesk installation, reads its RustDesk ID, attaches the shared
//! support access password (set once by the shop owner, with the client's
//! verbal consent at installation), and pushes everything to the owner's
//! Telegram — so the owner can connect directly without a phone call.
//! The OS browser is never involved and no data leaves beyond the owner's
//! own Telegram chat.

use crate::database::DbState;

/// Common RustDesk install locations on Windows (first match wins).
const RUSTDESK_PROBES: &[&str] = &[
    r"C:\Program Files\RustDesk\rustdesk.exe",
    r"C:\Program Files (x86)\RustDesk\rustdesk.exe",
];

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

pub fn probe_rustdesk() -> Option<std::path::PathBuf> {
    for p in RUSTDESK_PROBES {
        let path = std::path::PathBuf::from(p);
        if path.exists() {
            return Some(path);
        }
    }
    // Drop-in bundling: rustdesk.exe placed in a `support` folder next to
    // the installed TitaouPOS exe (one-setup path without installer edits).
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            for candidate in [
                dir.join("support").join("rustdesk.exe"),
                dir.join("rustdesk.exe"),
            ] {
                if candidate.exists() {
                    return Some(candidate);
                }
            }
        }
    }
    // Per-user installs.
    if let Some(lad) = std::env::var_os("LOCALAPPDATA") {
        let path = std::path::PathBuf::from(lad).join(r"RustDesk\rustdesk.exe");
        if path.exists() {
            return Some(path);
        }
    }
    None
}

/// One-click support request: RustDesk ID + access password → owner's Telegram.
pub fn request_support(db: &DbState) -> Result<String, String> {
    let settings = crate::services::settings_service::get_all_settings(db).unwrap_or_default();

    let exe: String = settings
        .get("rustdesk_path")
        .map(|p| p.trim().to_string())
        .filter(|p| !p.is_empty())
        .or_else(|| probe_rustdesk()?.to_string_lossy().to_string().into())
        .ok_or("RUSTDESK_MISSING")?;

    if !std::path::Path::new(&exe).exists() {
        return Err("RUSTDESK_MISSING".into());
    }

    // Read the RustDesk ID (blocking, local, fast; hidden console window).
    let id = {
        let mut cmd = std::process::Command::new(&exe);
        cmd.arg("--get-id");
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }
        let out = cmd
            .output()
            .map_err(|e| format!("RUSTDESK_RUN_FAILED: {}", e))?;
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    };
    if id.is_empty() {
        return Err("RUSTDESK_NO_ID".into());
    }

    let password = settings
        .get("rustdesk_support_password")
        .map(|p| p.trim().to_string())
        .unwrap_or_default();
    let password_line = if password.is_empty() {
        "—".to_string()
    } else {
        password.clone()
    };

    let pc = crate::network::terminal_name_for_this_pc();
    let lang = crate::services::notifier_service::ui_language(db);
    let text = crate::services::notifier_service::tr(
        &lang,
        (
            format!(
                "🆘 *Support Request* — PC: *{}*\n🖥 RustDesk ID: *{}*\n🔑 Access password: {}\nPlease connect remotely.",
                pc, id, password_line
            ),
            format!(
                "🆘 *طلب دعم* — الجهاز: *{}*\n🖥 معرّف RustDesk: *{}*\n🔑 كلمة المرور: {}\nيرجى الاتصال عن بعد.",
                pc, id, password_line
            ),
            format!(
                "🆘 *Demande d'assistance* — PC : *{}*\n🖥 ID RustDesk : *{}*\n🔑 Mot de passe : {}\nVeuillez vous connecter à distance.",
                pc, id, password_line
            ),
        ),
    );

    // The support request is an EXPLICIT user action: deliver it even when
    // the periodic alert switches are off. Telegram resolution order:
    //   1. the shop-wide bot config (synced to LAN-connected clients),
    //   2. the per-PC support credentials — for DISTANT PCs that never
    //      joined the shop network (the owner types them once at that PC).
    let (token, chat_id) =
        if let Some((token, chat_id, _)) = crate::services::notifier_service::get_telegram_config(db) {
            (token, chat_id)
        } else {
            let token = settings
                .get("support_telegram_token")
                .map(|t| t.trim().to_string())
                .unwrap_or_default();
            let chat_id = settings
                .get("support_telegram_chat_id")
                .map(|t| t.trim().to_string())
                .unwrap_or_default();
            if token.is_empty() || chat_id.is_empty() {
                return Err("NO_TELEGRAM".into());
            }
            (token, chat_id)
        };

    // Machine-readable marker: the owner's TitaouPOS polls this bot and
    // turns the message into an auto-connect card — works over the
    // INTERNET (RustDesk relays the session itself).
    let text = format!(
        "{}
#TITAOUSUPPORT|pc={}|id={}|pw={}",
        text, pc, id, password_line
    );

    // Fire-and-forget: never block the UI on the network.
    std::thread::spawn(move || {
        if let Err(e) =
            crate::services::notifier_service::send_telegram_blocking(&token, &chat_id, &text)
        {
            eprintln!("[support] telegram send failed: {}", e);
        }
    });

    Ok(id)
}

/// Owner side: launch the local RustDesk pointed at the requesting PC.
/// The RustDesk connect dialog opens; the password arrives with the
/// request toast for one-paste approval.
pub fn connect_rustdesk(rustdesk_id: &str, password: &str) -> Result<(), String> {
    if rustdesk_id.trim().is_empty() {
        return Err("RUSTDESK_NO_ID".into());
    }
    let exe = probe_rustdesk()
        .ok_or("RUSTDESK_MISSING")?
        .to_string_lossy()
        .to_string();
    let mut cmd = std::process::Command::new(&exe);
    cmd.arg("--connect").arg(rustdesk_id.trim());
    if !password.trim().is_empty() {
        cmd.arg("--password").arg(password.trim());
    }
    // The RustDesk window must be visible — no CREATE_NO_WINDOW here.
    cmd.spawn().map_err(|e| format!("RUSTDESK_RUN_FAILED: {}", e))?;
    Ok(())
}

/// Background poller on the OWNER's terminal: reads the shop bot's
/// getUpdates over the INTERNET and turns any #TITAOUSUPPORT message into
/// a local `support_requested` event — the same Connect Now card the LAN
/// path shows, but working from any distance. Runs every 12s; the first
/// pass drains the backlog silently so old requests never re-pop.
pub fn start_telegram_poller(db: DbState) {
    std::thread::Builder::new()
        .name("support-poller".into())
        .spawn(move || {
            let mut last_update_id: i64 = -1; // -1 = first pass: skip backlog
            loop {
                std::thread::sleep(std::time::Duration::from_secs(12));
                // Client-role terminals SEND requests; they never pop the
                // owner's Connect Now card. Self-originated messages are
                // ignored too (a distant PC must not connect to itself).
                if crate::network::is_client_role() {
                    continue;
                }
                let this_pc = crate::network::terminal_name_for_this_pc();
                let Some((token, _chat_id, _)) =
                    crate::services::notifier_service::get_telegram_config(&db)
                else {
                    continue; // Telegram not configured yet — retry quietly.
                };
                let offset = if last_update_id < 0 { -1i64 } else { last_update_id + 1 };
                let url = format!(
                    "https://api.telegram.org/bot{}/getUpdates?offset={}&limit=10&timeout=0",
                    token, offset
                );
                let Ok(client) = reqwest::blocking::Client::builder()
                    .user_agent("TitaouPOS-Support")
                    .timeout(std::time::Duration::from_secs(10))
                    .build()
                else {
                    continue;
                };
                let Ok(resp) = client.get(&url).send() else { continue };
                let Ok(body) = resp.json::<serde_json::Value>() else { continue };
                let Some(updates) = body.get("result").and_then(|r| r.as_array()) else { continue };
                let mut newest = last_update_id;
                for upd in updates {
                    let uid = upd.get("update_id").and_then(|v| v.as_i64()).unwrap_or(0);
                    if uid > newest {
                        newest = uid;
                    }
                    if last_update_id < 0 || uid <= last_update_id {
                        continue; // backlog drain or already handled
                    }
                    let text = upd
                        .get("message")
                        .and_then(|m| m.get("text"))
                        .and_then(|t| t.as_str())
                        .unwrap_or("");
                    if let Some(rest) = text.split("#TITAOUSUPPORT|").nth(1) {
                        let mut pc = String::new();
                        let mut rid = String::new();
                        let mut pw = String::new();
                        for part in rest.split('|') {
                            if let Some(v) = part.strip_prefix("pc=") {
                                pc = v.to_string();
                            } else if let Some(v) = part.strip_prefix("id=") {
                                rid = v.to_string();
                            } else if let Some(v) = part.strip_prefix("pw=") {
                                pw = v.to_string();
                            }
                        }
                        if !rid.is_empty() && pc != this_pc {
                            crate::network::emit_net_event(serde_json::json!({
                                "type": "support_requested",
                                "data": {
                                    "pc_name": pc,
                                    "rustdesk_id": rid,
                                    "password": pw,
                                },
                                "ts": std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .map(|d| d.as_secs())
                                    .unwrap_or(0),
                            }));
                            eprintln!("[support] internet request from {} (id {})", pc, rid);
                        }
                    }
                }
                if newest > last_update_id {
                    last_update_id = newest;
                }
            }
        })
        .expect("spawn support poller");
}
