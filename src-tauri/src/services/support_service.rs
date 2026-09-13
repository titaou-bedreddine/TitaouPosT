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
/// Where the auto-installed RustDesk lives (support folder next to the
/// installed TitaouPOS exe).
fn auto_install_dir() -> Option<std::path::PathBuf> {
    std::env::current_exe().ok()?.parent().map(|d| d.join("support"))
}

/// NO-MANUAL-INSTALL setup: resolve the latest official RustDesk release on
/// GitHub, download its Windows x64 portable zip, and extract rustdesk.exe
/// into the support folder next to TitaouPOS. After this, the Help button
/// works — the user never installs anything by hand.
pub async fn setup_rustdesk() -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(|| {
        let client = reqwest::blocking::Client::builder()
            .user_agent("TitaouPOS-Support")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| e.to_string())?;

        // 1. Latest release metadata.
        let meta: serde_json::Value = client
            .get("https://api.github.com/repos/rustdesk/rustdesk/releases/latest")
            .header("Accept", "application/vnd.github+json")
            .send()
            .map_err(|e| e.to_string())?
            .json()
            .map_err(|e| e.to_string())?;
        let assets = meta
            .get("assets")
            .and_then(|a| a.as_array())
            .ok_or("No assets in RustDesk release")?;
        let asset = assets
            .iter()
            .find(|a| {
                let name = a.get("name").and_then(|n| n.as_str()).unwrap_or("");
                name.contains("windows_x64") && name.ends_with(".zip") && !name.contains("sciter")
            })
            .ok_or("No windows_x64 zip in RustDesk release")?;
        let url = asset
            .get("browser_download_url")
            .and_then(|u| u.as_str())
            .ok_or("Asset has no download URL")?
            .to_string();
        let asset_name = asset.get("name").and_then(|n| n.as_str()).unwrap_or("rustdesk.zip");

        // 2. Download to a temp file (large — generous timeout).
        let dl = client
            .get(&url)
            .timeout(std::time::Duration::from_secs(600))
            .send()
            .map_err(|e| e.to_string())?;
        let zip_path = std::env::temp_dir().join(asset_name);
        let bytes = dl.bytes().map_err(|e| e.to_string())?;
        std::fs::write(&zip_path, &bytes).map_err(|e| e.to_string())?;

        // 3. Extract to the support folder next to TitaouPOS.
        let dest = auto_install_dir().ok_or("Cannot resolve install directory")?;
        std::fs::create_dir_all(&dest).map_err(|e| e.to_string())?;
        let status = std::process::Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                &format!(
                    "Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
                    zip_path.display(),
                    dest.display()
                ),
            ])
            .output()
            .map_err(|e| e.to_string())?;
        if !status.status.success() {
            return Err(format!(
                "Extract failed: {}",
                String::from_utf8_lossy(&status.stderr)
            ));
        }

        // 4. Verify rustdesk.exe exists somewhere in the extraction.
        let found = find_rustdesk_exe(&dest);
        let _ = std::fs::remove_file(&zip_path);
        match found {
            Some(p) => Ok(p.to_string_lossy().to_string()),
            None => Err("rustdesk.exe not found after extraction".into()),
        }
    })
    .await
    .map_err(|e| e.to_string())?
}

fn find_rustdesk_exe(dir: &std::path::Path) -> Option<std::path::PathBuf> {
    let direct = dir.join("rustdesk.exe");
    if direct.exists() {
        return Some(direct);
    }
    for entry in std::fs::read_dir(dir).ok()? {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(found) = find_rustdesk_exe(&path) {
                return Some(found);
            }
        }
    }
    None
}

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
                        if !rid.is_empty() && pc != this_pc && !crate::network::is_client_role() {
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
                // LICENSE markers: requests go to the owner; signed replies
                // activate the client whose HWID matches.
                for upd in updates {
                    let uid = upd.get("update_id").and_then(|v| v.as_i64()).unwrap_or(0);
                    let text = upd
                        .get("message")
                        .and_then(|m| m.get("text"))
                        .and_then(|t| t.as_str())
                        .unwrap_or("");
                    if text.contains("#TITAOUREQUEST|") && !crate::network::is_client_role() {
                        if let Some(rest) = text.split("#TITAOUREQUEST|").nth(1) {
                            let mut hw = String::new();
                            let mut pc = String::new();
                            let mut shop = String::new();
                            for part in rest.split('|') {
                                if let Some(v) = part.strip_prefix("hw=") { hw = v.to_string(); }
                                else if let Some(v) = part.strip_prefix("pc=") { pc = v.to_string(); }
                                else if let Some(v) = part.strip_prefix("shop=") { shop = v.to_string(); }
                            }
                            if !hw.is_empty() {
                                crate::network::emit_net_event(serde_json::json!({
                                    "type": "license_requested",
                                    "data": { "hwid": hw, "pc_name": pc, "shop": shop },
                                    "ts": std::time::SystemTime::now()
                                        .duration_since(std::time::UNIX_EPOCH)
                                        .map(|d| d.as_secs())
                                        .unwrap_or(0),
                                }));
                                eprintln!("[license] activation request from {} ({})", pc, hw);
                            }
                        }
                    }
                    if text.contains("#TITALICENSE|hw=") {
                        if let Some(rest) = text.split("#TITALICENSE|hw=").nth(1) {
                            let mut parts = rest.splitn(2, "|lic=");
                            let hw = parts.next().unwrap_or("").trim().to_uppercase();
                            let lic = parts.next().unwrap_or("").trim().to_string();
                            let this_hw = crate::services::license_service::get_hwid().to_uppercase();
                            if hw == this_hw && !lic.is_empty() {
                                match crate::services::license_service::verify_and_activate(&db, &lic) {
                                    Ok((mode, expiry, shop)) => {
                                        crate::network::emit_net_event(serde_json::json!({
                                            "type": "license_activated",
                                            "data": { "mode": mode, "expiry": expiry, "shop": shop },
                                            "ts": std::time::SystemTime::now()
                                                .duration_since(std::time::UNIX_EPOCH)
                                                .map(|d| d.as_secs())
                                                .unwrap_or(0),
                                        }));
                                        eprintln!("[license] ACTIVATED via Telegram ({})", mode);
                                    }
                                    Err(e) => eprintln!("[license] activation rejected: {}", e),
                                }
                            }
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
