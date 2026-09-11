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

fn probe_rustdesk() -> Option<std::path::PathBuf> {
    for p in RUSTDESK_PROBES {
        let path = std::path::PathBuf::from(p);
        if path.exists() {
            return Some(path);
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
    // the periodic alert switches are off — but Telegram must be configured.
    let Some((token, chat_id, _)) = crate::services::notifier_service::get_telegram_config(db) else {
        return Err("NO_TELEGRAM".into());
    };

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
