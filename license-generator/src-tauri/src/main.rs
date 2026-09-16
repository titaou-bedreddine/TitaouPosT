//! TitaouPOS License Generator — the DEVELOPER's standalone tool.
//!
//! Mirrors the POS app's license_service crypto exactly (same payload
//! shape, same minisign pubkey embedding) so every key it signs activates
//! in the POS:
//! - Paste a client's request code (from the POS setup wizard / Activation
//!   tab) → parse shop, owner and HWID in one shot.
//! - Sign Full (lifetime) or Trial (N days) → copyable serial key +
//!   downloadable .lic file.
//! - Publish the license to the GitHub registry repo (online activation
//!   for the client: one button, no typing on their side).
//! - Revoke a HWID → revoked.json in the registry; every POS refuses that
//!   machine within 30 minutes (or at its next activation).

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine;
use minisign::KeyPair;
use serde_json::json;
use tauri::Manager;

/// MUST equal the POS app's embedded public key (license_service.rs).
pub const LICENSE_PUBKEY: &str = "RWTLZ4gRtN0qIYbsbBblb34+DOspf2drcCUKrPkTj+tMRIWDQCvDmjMv";

pub const REGISTRY_OWNER: &str = "titaou-bedreddine";
pub const REGISTRY_REPO: &str = "TitaouPosT-licenses";

/// Where the master keypair lives on THIS (developer) machine:
/// %APPDATA%\TitaouPosT\license_master.key — shared with the in-app
/// License Studio so both tools sign with the same key.
fn master_key_path() -> std::path::PathBuf {
    let base = std::env::var_os("APPDATA")
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(std::path::PathBuf::from))
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    base.join("TitaouPosT").join("license_master.key")
}

fn master_key_exists() -> bool {
    master_key_path().exists()
}

/// Generate the master keypair (once): license_master.key (SECRET — never
/// share, never commit) + license_master.pub next to it.
#[tauri::command]
fn generate_master_keypair() -> Result<String, String> {
    let kp = KeyPair::generate_unencrypted_keypair().map_err(|e| e.to_string())?;
    let path = master_key_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    // Minisign file format: comment line + base64 key blob.
    std::fs::write(
        &path,
        format!(
            "untrusted comment: TitaouPOS license secret key\n{}\n",
            B64.encode(kp.sk.to_bytes())
        ),
    )
    .map_err(|e| e.to_string())?;
    std::fs::write(
        path.with_extension("pub"),
        format!(
            "untrusted comment: TitaouPOS license public key\n{}\n",
            B64.encode(kp.pk.to_bytes())
        ),
    )
    .map_err(|e| e.to_string())?;
    Ok(kp.pk.to_base64())
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct LicensePayload {
    pub shop: String,
    pub hwid: String,
    pub mode: String,
    pub expiry: Option<String>,
    pub terminals: i64,
    pub issued: String,
    pub lid: String,
}

fn today() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

/// Sign a license for (shop, hwid, mode). Returns
/// { key, lic, hwid, shop, mode, expiry } — the UI shows/copies/downloads.
#[tauri::command]
fn create_license(
    shop_name: String,
    hwid: String,
    owner: String,
    mode: String,
    days: i64,
) -> Result<serde_json::Value, String> {
    if shop_name.trim().is_empty() || hwid.trim().is_empty() {
        return Err("Shop name and HWID are required".into());
    }
    let key_path = master_key_path();
    if !key_path.exists() {
        return Err("NO_MASTER_KEY".into());
    }
    let mode = if mode == "trial" { "trial" } else { "full" };
    let expiry = if mode == "trial" {
        Some(
            (chrono::Local::now() + chrono::Duration::days(days.max(1)))
                .format("%Y-%m-%d")
                .to_string(),
        )
    } else {
        None
    };
    let payload = LicensePayload {
        shop: shop_name.trim().to_string(),
        hwid: hwid.trim().to_uppercase(),
        mode: mode.to_string(),
        expiry,
        terminals: 1,
        issued: today(),
        lid: format!("LIC-{}", chrono::Local::now().format("%Y%m%d%H%M%S")),
    };
    let payload_json = serde_json::to_string(&payload).map_err(|e| e.to_string())?;

    // Some("") — explicit EMPTY password: passing None makes minisign try
    // an INTERACTIVE console prompt, which fails in a GUI app ("handle is
    // invalid"). Our master key is unencrypted, so the empty password is
    // the correct non-interactive path.
    let sk = minisign::SecretKey::from_file(&key_path, Some(String::new()))
        .map_err(|e| format!("load master key: {}", e))?;
    let pk = minisign::PublicKey::from_base64(LICENSE_PUBKEY)
        .map_err(|e| format!("pubkey mismatch with the POS build: {}", e))?;
    let sig_box = minisign::sign(
        Some(&pk),
        &sk,
        std::io::Cursor::new(payload_json.as_bytes()),
        Some("trusted comment: TitaouPOS license"),
        Some(&format!(
            "License for {} / {} ({})",
            payload.shop,
            owner.trim(),
            payload.hwid
        )),
    )
    .map_err(|e| format!("sign failed: {}", e))?;
    let box_string = sig_box.to_string();

    let key_string = format!(
        "{}.{}",
        B64.encode(payload_json.as_bytes()),
        B64.encode(box_string.as_bytes())
    );
    let lic_file = format!("{}\n{}", payload_json, box_string);
    Ok(json!({
        "key": key_string,
        "lic": lic_file,
        "hwid": payload.hwid,
        "shop": payload.shop,
        "mode": payload.mode,
        "expiry": payload.expiry,
    }))
}

/// Parse what the developer pastes — tolerant to all client formats:
/// - TIT-REQ|v=1|hw=HW-..|shop=..|owner=..  (setup wizard / Activation page)
/// - #TITAOUREQUEST|hw=..|pc=..|shop=..    (Telegram request line)
/// - a bare HW-XXXXXXXXXXXXXXXX machine id
#[tauri::command]
fn parse_request_code(code: String) -> Result<serde_json::Value, String> {
    let text = code.trim();
    let mut hw = String::new();
    let mut shop = String::new();
    let mut owner = String::new();
    // One pass over k=v pairs covers both pipe formats.
    let inner = text
        .strip_prefix("TIT-REQ|")
        .or_else(|| text.split_once("#TITAOUREQUEST|").map(|(_, rest)| rest))
        .map(|s| s.to_string());
    if let Some(inner) = inner {
        for part in inner.split('|') {
            if let Some(v) = part.strip_prefix("hw=") {
                hw = v.trim().to_string();
            } else if let Some(v) = part.strip_prefix("shop=") {
                shop = v.trim().to_string();
            } else if let Some(v) = part.strip_prefix("owner=") {
                owner = v.trim().to_string();
            }
        }
    }
    // Fallback: scan for a bare HW-... machine id anywhere in the text.
    if hw.is_empty() {
        if let Some(pos) = text.find("HW-") {
            let end = text[pos..]
                .find(|c: char| !c.is_ascii_alphanumeric() && c != '-')
                .map(|i| pos + i)
                .unwrap_or(text.len());
            hw = text[pos..end].to_string();
        }
    }
    if hw.is_empty() {
        return Err(format!(
            "No machine ID found — paste the client's request code (TIT-REQ|…), the Telegram request line, or a HW-… id.\nGot: {}",
            if text.len() > 80 { &text[..80] } else { text }
        ));
    }
    Ok(json!({ "hwid": hw, "shop": shop, "owner": owner }))
}

// ---------------------------------------------------------------------------
// GitHub registry (online activation + revocation)
// ---------------------------------------------------------------------------

fn gh_token() -> Result<String, String> {
    // 1. env var (CI or manual), 2. gh CLI keyring token via `gh auth token`.
    if let Ok(t) = std::env::var("GITHUB_TOKEN") {
        if !t.trim().is_empty() {
            return Ok(t.trim().to_string());
        }
    }
    let out = std::process::Command::new("gh")
        .args(["auth", "token"])
        .output()
        .map_err(|e| format!("gh CLI not available: {}", e))?;
    if !out.status.success() {
        return Err("gh auth token failed — run `gh auth login` once".into());
    }
    let token = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if token.is_empty() {
        return Err("empty GitHub token".into());
    }
    Ok(token)
}

/// Publish licenses/<HWID>.json to the registry (PUT contents API) so the
/// client's "Activate This PC Online" button pulls it. Repo must exist.
#[tauri::command]
fn publish_license(hwid: String, license_key: String, shop: String, mode: String) -> Result<String, String> {
    let token = gh_token()?;
    let hw = hwid.trim().to_uppercase();
    let body = json!({
        "licensed": true,
        "license_key": license_key.trim(),
        "shop": shop.trim(),
        "mode": mode.trim(),
        "published_at": chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    });
    put_registry_file(
        &token,
        &format!("licenses/{}.json", hw),
        &serde_json::to_string_pretty(&body).unwrap(),
        &format!("license for {} ({})", shop.trim(), hw),
    )
}

/// Add HWID to revoked.json (never used again on any other PC — the POS
/// poller flips the machine to read-only within 30 minutes). Re-issuing a
/// NEW signed license removes the revocation.
#[tauri::command]
fn revoke_license(hwid: String) -> Result<String, String> {
    let token = gh_token()?;
    let hw = hwid.trim().to_uppercase();
    let current = get_registry_file(&token, "revoked.json")
        .map(|(c, _)| c)
        .unwrap_or_else(|_| "{\"revoked\": []}".to_string());
    let mut v: serde_json::Value =
        serde_json::from_str(&current).unwrap_or(json!({"revoked": []}));
    let arr = v
        .as_object_mut()
        .ok_or("revoked.json malformed")?
        .entry("revoked")
        .or_insert(json!([]));
    if let Some(list) = arr.as_array_mut() {
        if !list.iter().any(|x| x.as_str() == Some(hw.as_str())) {
            list.push(json!(hw));
        }
    }
    put_registry_file(
        &token,
        "revoked.json",
        &serde_json::to_string_pretty(&v).unwrap(),
        &format!("revoke {}", hw),
    )
}

/// Remove a HWID from revoked.json (when a new license is re-issued).
#[tauri::command]
fn un_revoke_license(hwid: String) -> Result<String, String> {
    let token = gh_token()?;
    let hw = hwid.trim().to_uppercase();
    let current = get_registry_file(&token, "revoked.json")
        .map(|(c, _)| c)
        .unwrap_or_else(|_| "{\"revoked\": []}".to_string());
    let mut v: serde_json::Value =
        serde_json::from_str(&current).unwrap_or(json!({"revoked": []}));
    if let Some(list) = v.get_mut("revoked").and_then(|a| a.as_array_mut()) {
        list.retain(|x| x.as_str() != Some(hw.as_str()));
    }
    put_registry_file(
        &token,
        "revoked.json",
        &serde_json::to_string_pretty(&v).unwrap(),
        &format!("un-revoke {}", hw),
    )
}

/// Read the current revoked list (for the UI's revocation manager).
#[tauri::command]
fn list_revoked() -> Result<serde_json::Value, String> {
    let token = gh_token()?;
    match get_registry_file(&token, "revoked.json") {
        Ok((content, _)) => {
            let v: serde_json::Value = serde_json::from_str(&content).unwrap_or(json!({"revoked": []}));
            Ok(v)
        }
        Err(_) => Ok(json!({ "revoked": [] })),
    }
}

// --- GitHub contents API helpers (repo: titaou-bedreddine/TitaouPosT-licenses)

fn api_url(path: &str) -> String {
    format!(
        "https://api.github.com/repos/{}/{}/contents/{}",
        REGISTRY_OWNER, REGISTRY_REPO, path
    )
}

fn get_registry_file(token: &str, path: &str) -> Result<(String, String), String> {
    let resp = reqwest::blocking::Client::builder()
        .user_agent("TitaouPOS-LicenseGenerator")
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(|e| e.to_string())?
        .get(api_url(path))
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/vnd.github+json")
        .send()
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("registry read failed ({})", resp.status()));
    }
    let v: serde_json::Value = resp.json().map_err(|e| e.to_string())?;
    let sha = v
        .get("sha")
        .and_then(|s| s.as_str())
        .unwrap_or("")
        .to_string();
    let b64 = v.get("content").and_then(|c| c.as_str()).unwrap_or("");
    let content = String::from_utf8_lossy(
        &B64.decode(b64.replace('\n', "")).unwrap_or_default(),
    )
    .to_string();
    Ok((content, sha))
}

fn put_registry_file(token: &str, path: &str, content: &str, msg: &str) -> Result<String, String> {
    let sha = get_registry_file(token, path)
        .map(|(_, s)| s)
        .ok();
    let body = json!({
        "message": msg,
        "content": B64.encode(content.as_bytes()),
        "branch": "main",
        "sha": sha,
    });
    let client = reqwest::blocking::Client::builder()
        .user_agent("TitaouPOS-LicenseGenerator")
        .timeout(std::time::Duration::from_secs(20))
        .build()
        .map_err(|e| e.to_string())?;
    let mut req = client
        .put(api_url(path))
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/vnd.github+json")
        .json(&body);
    if let Some(s) = &sha {
        if s.is_empty() {
            req = client
                .put(api_url(path))
                .header("Authorization", format!("Bearer {}", token))
                .header("Accept", "application/vnd.github+json")
                .json(&body);
        }
    }
    let resp = req.send().map_err(|e| e.to_string())?;
    let status = resp.status();
    let text = resp.text().unwrap_or_default();
    if !status.is_success() {
        return Err(format!("registry publish failed ({}): {}", status, text));
    }
    Ok(format!("Published {} → {}/{}", msg, REGISTRY_OWNER, REGISTRY_REPO))
}

#[tauri::command]
fn master_status() -> serde_json::Value {
    json!({ "exists": master_key_exists(), "path": master_key_path().to_string_lossy() })
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let win = app.get_webview_window("main").unwrap();
            // Keep the master key path visible to the frontend.
            win.eval(&format!(
                "window.__MASTER_KEY_PATH__ = {};",
                serde_json::to_string(&master_key_path().to_string_lossy()).unwrap()
            ))
            .ok();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            generate_master_keypair,
            master_status,
            create_license,
            parse_request_code,
            publish_license,
            revoke_license,
            un_revoke_license,
            list_revoked,
        ])
        .run(tauri::generate_context!())
        .expect("error while running license generator");
}
