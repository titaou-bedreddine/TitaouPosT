//! Cryptographic licensing (v0.5.34) + enforcement & revocation (v0.6.0).
//!
//! Licenses are Ed25519 minisign signatures over a JSON payload bound to
//! the machine HWID. The shop owner holds the master SECRET key; the
//! PUBLIC key is compiled into this app, so clients can verify but never
//! forge licenses — even with full access to the bot token or database.
//!
//! Key string format (pasteable):  base64(payload_json) + "." + base64(signature_box)
//! .lic file format:               payload_json + "\n" + minisign signature box
//! Trial licenses carry an expiry date; full licenses none.
//!
//! v0.6.0 ENFORCEMENT:
//! - "no license = read-only": every business mutation (sales, products,
//!   purchases, cash, ...) is refused while this terminal has no ACTIVE
//!   signed license. Reads, settings and the activation surface stay open
//!   so the shop can be configured and licensed.
//! - `app_license_*` settings are writable ONLY through this service (the
//!   generic set_setting drops them), so no UI/API surface can forge a
//!   license state; the stored key is re-verified at every boot.
//! - Revocation: `revoked.json` in the public GitHub registry repo lists
//!   HWIDs whose license is dead (checked at activation + every 30 min).
//! - Online activation: `licenses/<HWID>.json` in the same repo carries a
//!   real signed key published by the developer's License Generator.

use crate::database::DbState;
use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine;
use serde_json::json;

/// Trusted public key (minisign, base64). Generated with the master
/// keypair; rotate by regenerating on the dev machine and replacing this
/// constant in the next build.
pub const LICENSE_PUBKEY: &str = "RWTLZ4gRtN0qIYbsbBblb34+DOspf2drcCUKrPkTj+tMRIWDQCvDmjMv";

/// Public GitHub registry (TitaouPosT-licenses): licenses/<HWID>.json for
/// online activation, revoked.json for revocation. Maintained by the
/// developer's standalone License Generator.
pub const LICENSES_REPO_OWNER: &str = "titaou-bedreddine";
pub const LICENSES_REPO_NAME: &str = "TitaouPosT-licenses";

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct LicensePayload {
    pub shop: String,
    pub hwid: String,
    /// "full" | "trial"
    pub mode: String,
    /// YYYY-MM-DD — trial end date (null for full).
    pub expiry: Option<String>,
    pub terminals: i64,
    pub issued: String,
    pub lid: String,
}

fn today() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

/// Generate the master keypair: writes license_master.key (SECRET — the
/// developer keeps this file private) and license_master.pub next to the
/// app database, and returns the public key base64 (to embed in the next
/// build when rotating).
pub fn generate_master_keypair() -> Result<(String, String, String), String> {
    let kp = minisign::KeyPair::generate_unencrypted_keypair()
        .map_err(|e| format!("keygen failed: {}", e))?;
    let db_dir = crate::database::get_database_path()
        .parent()
        .ok_or("no db dir")?
        .to_path_buf();
    std::fs::create_dir_all(&db_dir).map_err(|e| e.to_string())?;
    let sk_path = db_dir.join("license_master.key");
    let pk_path = db_dir.join("license_master.pub");
    // Minisign file format: comment line + base64 of the key blob.
    let sk_file = format!(
        "untrusted comment: TitaouPOS license secret key
{}
",
        B64.encode(kp.sk.to_bytes())
    );
    let pk_file = format!(
        "untrusted comment: TitaouPOS license public key
{}
",
        B64.encode(kp.pk.to_bytes())
    );
    std::fs::write(&sk_path, sk_file).map_err(|e| e.to_string())?;
    std::fs::write(&pk_path, pk_file).map_err(|e| e.to_string())?;
    Ok((
        kp.pk.to_base64(),
        sk_path.to_string_lossy().to_string(),
        pk_path.to_string_lossy().to_string(),
    ))
}

fn master_key_path(db: &DbState) -> Result<std::path::PathBuf, String> {
    let settings = crate::services::settings_service::get_all_settings(db).unwrap_or_default();
    let path = settings
        .get("license_master_path")
        .map(|p| p.trim().to_string())
        .unwrap_or_default();
    if path.is_empty() {
        return Err("Master key not configured — generate it first".into());
    }
    let p = std::path::PathBuf::from(&path);
    if !p.exists() {
        return Err("Master key file is missing".into());
    }
    Ok(p)
}

/// Create a signed license for a client. Returns (key_string, lic_file_content).
pub fn create_license(
    db: &DbState,
    shop_name: &str,
    hwid: &str,
    mode: &str,
    days: i64,
) -> Result<(String, String), String> {
    let key_path = master_key_path(db)?;
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
        shop: shop_name.to_string(),
        hwid: hwid.trim().to_uppercase(),
        mode: mode.to_string(),
        expiry,
        terminals: 1,
        issued: today(),
        lid: format!("LIC-{}", chrono::Local::now().format("%Y%m%d%H%M%S")),
    };
    let payload_json = serde_json::to_string(&payload).map_err(|e| e.to_string())?;

    let sk = minisign::SecretKey::from_file(&key_path, None)
        .map_err(|e| format!("load master key: {}", e))?;
    let pk = minisign::PublicKey::from_base64(LICENSE_PUBKEY)
        .map_err(|e| format!("embedded pubkey invalid: {}", e))?;

    let sig_box = minisign::sign(
        Some(&pk),
        &sk,
        std::io::Cursor::new(payload_json.as_bytes()),
        Some("trusted comment: TitaouPOS license"),
        Some(&format!("License for {} ({})", shop_name, payload.hwid)),
    )
    .map_err(|e| format!("sign failed: {}", e))?;
    let box_string = sig_box.to_string();

    let payload_b64 = B64.encode(payload_json.as_bytes());
    let sig_b64 = B64.encode(box_string.as_bytes());
    let key_string = format!("{}.{}", payload_b64, sig_b64);
    let lic_file = format!("{}\n{}", payload_json, box_string);

    Ok((key_string, lic_file))
}

enum ParsedLicense {
    Compact(Vec<u8>, String),
    File(Vec<u8>, String),
}

fn parse_license_text(text: &str) -> Result<ParsedLicense, String> {
    let text = text.trim();
    if text.contains("untrusted comment:") {
        let idx = text
            .find("\nuntrusted comment:")
            .ok_or("Malformed .lic file")?;
        Ok(ParsedLicense::File(
            text[..idx].as_bytes().to_vec(),
            text[idx + 1..].to_string(),
        ))
    } else if let Some((p, sig)) = text.split_once('.') {
        let payload = B64.decode(p.trim()).map_err(|e| format!("bad key payload: {}", e))?;
        let box_text = B64.decode(sig.trim())
            .ok()
            .and_then(|b| String::from_utf8(b).ok())
            .ok_or("bad key signature")?;
        Ok(ParsedLicense::Compact(payload, box_text))
    } else {
        Err("Unrecognized license format".into())
    }
}

/// Write one license setting DIRECTLY — the only path allowed to touch
/// `app_license_*` keys (the generic set_setting silently drops them so no
/// UI / LAN / restored-backup surface can forge a license state).
pub(crate) fn set_license_setting(db: &DbState, key: &str, value: &str) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();
    conn.execute(
        "INSERT INTO app_settings (key, value, updated_at) VALUES (?1, ?2, CURRENT_TIMESTAMP)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = CURRENT_TIMESTAMP",
        rusqlite::params![key, value],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// Signature + machine + expiry check WITHOUT persisting anything.
fn verify_only(license_text: &str) -> Result<LicensePayload, String> {
    let (payload_bytes, box_string) = match parse_license_text(license_text)? {
        ParsedLicense::Compact(p, b) => (p, b),
        ParsedLicense::File(p, b) => (p, b),
    };
    let pk = minisign_verify::PublicKey::from_base64(LICENSE_PUBKEY)
        .map_err(|e| format!("embedded pubkey invalid: {}", e))?;
    let sig = minisign_verify::Signature::decode(&box_string)
        .map_err(|e| format!("bad signature block: {}", e))?;
    pk.verify(&payload_bytes, &sig, true)
        .map_err(|_| "License signature is INVALID — reject".to_string())?;

    let payload: LicensePayload = serde_json::from_slice(&payload_bytes).map_err(|e| e.to_string())?;
    let this_hwid = get_hwid().to_uppercase();
    if payload.hwid.trim().to_uppercase() != this_hwid {
        return Err(format!(
            "This license is bound to another machine ({})",
            payload.hwid
        ));
    }
    if payload.mode == "trial" {
        if let Some(exp) = &payload.expiry {
            if today() > *exp {
                return Err(format!("Trial license expired on {}", exp));
            }
        }
    }
    Ok(payload)
}

/// Verify a pasted key or uploaded .lic against THIS machine and activate.
/// Returns (mode, expiry, shop).
pub fn verify_and_activate(db: &DbState, license_text: &str) -> Result<(String, Option<String>, String), String> {
    let payload = verify_only(license_text)?;

    // Revocation: online check before persisting; offline = fail-open (the
    // signature already proves authenticity; revocation needs the network,
    // and the 30-min poller catches it as soon as connectivity returns).
    if let Ok(list) = fetch_revoked_list() {
        if list.iter().any(|h| h.trim().eq_ignore_ascii_case(&payload.hwid)) {
            return Err("This license has been REVOKED by the developer".into());
        }
    }

    set_license_setting(db, "app_license_key", license_text)?;
    set_license_setting(db, "app_license_status", &payload.mode)?;
    set_license_setting(db, "app_license_expiry", payload.expiry.as_deref().unwrap_or(""))?;
    set_license_setting(db, "app_license_shop", &payload.shop)?;
    Ok((payload.mode, payload.expiry.clone(), payload.shop.clone()))
}

/// Current license state for the UI (trial expiry enforced here).
/// v0.6.0: the default is "none" — NOT "activated". Legacy manual
/// activations ("activated") map to "full".
pub fn license_status(db: &DbState) -> Result<serde_json::Value, String> {
    let settings = crate::services::settings_service::get_all_settings(db).unwrap_or_default();
    let mut status = settings
        .get("app_license_status")
        .cloned()
        .unwrap_or_else(|| "none".into());
    if status == "activated" {
        status = "full".into(); // legacy manual code (pre-signed era)
    }
    let expiry = settings.get("app_license_expiry").cloned().unwrap_or_default();
    let shop = settings.get("app_license_shop").cloned().unwrap_or_default();
    if status == "trial" && !expiry.is_empty() && today() > expiry {
        status = "expired".into();
    }
    let readonly = !matches!(status.as_str(), "full" | "trial");
    Ok(serde_json::json!({
        "status": status,
        "expiry": expiry,
        "shop": shop,
        "hwid": get_hwid(),
        "readonly": readonly,
    }))
}

pub fn get_hwid() -> String {
    crate::services::settings_service::get_hwid()
}

// ---------------------------------------------------------------------------
// READ-ONLY ENFORCEMENT (v0.6.0): no license = no business mutations.
// ---------------------------------------------------------------------------

/// Business mutations refused in read-only mode. Deliberately excludes:
/// reads, settings (the setup wizard + activation must work pre-license —
/// license keys themselves are protected at the setter), the licensing
/// surface (activate/studio/telegram) and support/update commands.
pub const MUTATING_COMMANDS: &[&str] = &[
    // sales
    "process_sale", "create_sale", "replace_sale", "hold_sale",
    "delete_held_sale", "delete_sale",
    // products & catalog
    "save_product", "delete_product", "save_category", "delete_category",
    "save_unit", "toggle_product_pin", "reorder_pinned_products",
    "save_packagings", "upload_product_to_scale", "upload_all_scalable_to_scale",
    "fetch_products_from_scale",
    // purchases
    "create_purchase", "update_purchase", "delete_purchase",
    // expenses
    "add_expense", "update_expense", "delete_expense",
    // cash register
    "open_cash_session", "add_cash_movement", "close_cash_session",
    "edit_opening_balance", "edit_cash_session", "archive_cash_session",
    "delete_cash_session",
    // customers & suppliers
    "save_customer", "delete_customer", "toggle_customer_pin",
    "record_customer_debt_payment", "clear_customer_debt",
    "save_supplier", "delete_supplier", "toggle_supplier_pin",
    "record_supplier_debt_payment", "clear_supplier_debt",
    // employees & payroll
    "save_employee", "delete_employee", "record_employee_advance",
    "record_employee_absence",
    // users
    "create_user", "update_user", "delete_user", "toggle_user_pin",
    "change_user_password",
    // destructive
    "clear_transaction_history", "factory_reset", "restore_database",
    "restore_settings_only",
];

pub fn is_gated_command(cmd: &str) -> bool {
    MUTATING_COMMANDS.contains(&cmd)
}

/// The gate: Ok = allowed, Err = blocked with a message the UI recognizes
/// (APP_READ_ONLY prefix). `full` and valid `trial` pass; none/expired/
/// revoked are read-only.
pub fn gate_command(db: &DbState, _cmd: &str) -> Result<(), String> {
    let status = license_status(db)?;
    let s = status.get("status").and_then(|v| v.as_str()).unwrap_or("none");
    if matches!(s, "full" | "trial") {
        return Ok(());
    }
    let why = match s {
        "expired" => "trial license expired",
        "revoked" => "license REVOKED",
        _ => "no active license",
    };
    Err(format!(
        "APP_READ_ONLY: {} — this terminal is READ-ONLY. Activate it in Settings → Activation.",
        why
    ))
}

/// Gate DB shared with the invoke wrapper (the Tauri 2.0 Invoke struct
/// carries no window/app handle, so the managed state is unreachable
/// there — same dedicated-connection pattern the pollers use).
static GATE_DB: std::sync::OnceLock<DbState> = std::sync::OnceLock::new();

pub fn set_gate_db(db: DbState) {
    let _ = GATE_DB.set(db);
}

/// Gate a command using the shared gate DB (invoke wrapper entry point).
pub fn gate_current(cmd: &str) -> Result<(), String> {
    match GATE_DB.get() {
        Some(db) => gate_command(db, cmd),
        None => Ok(()), // gate DB not wired yet — startup path, allow
    }
}

/// Boot-time revalidation: the stored `app_license_key` (if any) is
/// re-verified against the embedded pubkey + this HWID + expiry, and the
/// status is rebuilt FROM the key. A DB-edited `app_license_status`
/// without a valid signed key is wiped back to "none" — except the
/// legacy pre-signed "activated" flag (grandfathered: those machines
/// activated before signing existed, and their status maps to full).
pub fn revalidate_stored_license(db: &DbState) {
    let settings = crate::services::settings_service::get_all_settings(db).unwrap_or_default();
    let key = settings
        .get("app_license_key")
        .cloned()
        .unwrap_or_default();
    let cur = settings.get("app_license_status").cloned().unwrap_or_default();
    if key.trim().is_empty() {
        if cur == "activated" {
            // legacy flag without a key — grandfathered pre-signed machine
            let _ = set_license_setting(db, "app_license_status", "full");
        } else if !cur.is_empty() && cur != "none" {
            // status claimed without any key at all — tampered / orphaned
            let _ = set_license_setting(db, "app_license_status", "none");
        }
        return;
    }
    match verify_only(&key) {
        Ok(payload) => {
            let _ = set_license_setting(db, "app_license_status", &payload.mode);
            let _ = set_license_setting(
                db,
                "app_license_expiry",
                payload.expiry.as_deref().unwrap_or(""),
            );
            let _ = set_license_setting(db, "app_license_shop", &payload.shop);
        }
        Err(_) => {
            // Tampered / foreign-machine / expired key: back to read-only.
            let _ = set_license_setting(db, "app_license_status", "none");
            let _ = set_license_setting(db, "app_license_expiry", "");
            let _ = set_license_setting(db, "app_license_shop", "");
        }
    }
}

// ---------------------------------------------------------------------------
// Revocation registry (public GitHub repo, maintaied by the License
// Generator's "Revoke" tool).
// ---------------------------------------------------------------------------

fn raw_registry_url(file: &str) -> String {
    format!(
        "https://raw.githubusercontent.com/{}/{}/main/{}",
        LICENSES_REPO_OWNER, LICENSES_REPO_NAME, file
    )
}

/// Fetch the revocation list. Accepts {"revoked": ["HW-..", ..]} or a bare
/// array. Errors only on network/parse failure (callers fail open).
pub fn fetch_revoked_list() -> Result<Vec<String>, String> {
    let body: serde_json::Value = reqwest::blocking::Client::builder()
        .user_agent("TitaouPOS-License")
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?
        .get(raw_registry_url("revoked.json"))
        .send()
        .map_err(|e| e.to_string())?
        .json()
        .map_err(|e| e.to_string())?;
    let arr = body
        .get("revoked")
        .and_then(|v| v.as_array())
        .cloned()
        .or_else(|| body.as_array().cloned())
        .unwrap_or_default();
    Ok(arr
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.trim().to_string()))
        .filter(|s| !s.is_empty())
        .collect())
}

/// If this machine appears in the revocation list, flip its status to
/// "revoked" (one-way — only a NEW signed license reactivates it) and
/// notify the UI. Offline = keep the current state.
pub fn check_revocation(db: &DbState) {
    let Ok(list) = fetch_revoked_list() else { return };
    let hw = get_hwid().to_uppercase();
    if list.iter().any(|h| h.eq_ignore_ascii_case(&hw)) {
        let cur = license_status(db)
            .ok()
            .and_then(|v| v.get("status").and_then(|s| s.as_str().map(String::from)))
            .unwrap_or_default();
        if cur == "revoked" {
            return;
        }
        let _ = set_license_setting(db, "app_license_status", "revoked");
        crate::network::emit_net_event(serde_json::json!({
            "type": "license_revoked",
            "data": {},
            "ts": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        }));
        eprintln!("[license] REVOKED via registry — terminal is read-only");
    }
}

/// Background thread: check the revocation registry shortly after boot,
/// then every 30 minutes (offline = silently retries).
pub fn start_revocation_poller(db: DbState) {
    std::thread::Builder::new()
        .name("license-revocation".into())
        .spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(30));
            loop {
                check_revocation(&db);
                std::thread::sleep(std::time::Duration::from_secs(30 * 60));
            }
        })
        .expect("spawn license revocation poller");
}

// ---------------------------------------------------------------------------
// Online activation from the GitHub registry (published by the License
// Generator). The registry entry is transport; the SIGNATURE is the trust.
// ---------------------------------------------------------------------------

/// Fetch `licenses/<HWID>.json` — must carry {"licensed": true,
/// "license_key": "<signed key or .lic body>"} — and activate through the
/// same verification path as a pasted key. Err("NO_ONLINE_LICENSE: ...")
/// when this PC has no published license yet (the UI falls back to a
/// Telegram request).
pub fn activate_from_registry(db: &DbState) -> Result<serde_json::Value, String> {
    let hwid = get_hwid();
    let resp = reqwest::blocking::Client::builder()
        .user_agent("TitaouPOS-License")
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(|e| format!("NO_ONLINE_LICENSE: {}", e))?
        .get(raw_registry_url(&format!("licenses/{}.json", hwid)))
        .send()
        .map_err(|e| format!("NO_ONLINE_LICENSE: unreachable ({})", e))?;
    if !resp.status().is_success() {
        return Err("NO_ONLINE_LICENSE: no license published for this PC yet".into());
    }
    let body: serde_json::Value = resp
        .json()
        .map_err(|_| "NO_ONLINE_LICENSE: bad registry entry".to_string())?;
    if !body.get("licensed").and_then(|v| v.as_bool()).unwrap_or(false) {
        return Err("NO_ONLINE_LICENSE: registry entry is not active".into());
    }
    let key = body
        .get("license_key")
        .and_then(|v| v.as_str())
        .ok_or("NO_ONLINE_LICENSE: registry entry has no license key")?
        .to_string();
    let (mode, expiry, shop) = verify_and_activate(db, &key)?;
    Ok(json!({ "mode": mode, "expiry": expiry, "shop": shop }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    static DB_SEQ: AtomicU32 = AtomicU32::new(0);

    fn test_db() -> DbState {
        let dir = std::env::temp_dir().join("titaou_license_tests");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join(format!(
            "lic_{}_{}.sqlite",
            std::process::id(),
            DB_SEQ.fetch_add(1, Ordering::SeqCst)
        ));
        let conn = rusqlite::Connection::open(&path).unwrap();
        conn.execute_batch(
            "CREATE TABLE app_settings (
                key TEXT PRIMARY KEY,
                value TEXT,
                updated_at TEXT
            );",
        )
        .unwrap();
        DbState {
            conn: std::sync::Mutex::new(conn),
        }
    }

    #[test]
    fn fresh_install_is_none_and_gates_mutations() {
        let db = test_db();
        let st = license_status(&db).unwrap();
        assert_eq!(st["status"].as_str().unwrap(), "none");
        assert_eq!(st["readonly"].as_bool().unwrap(), true);
        assert!(gate_command(&db, "process_sale").unwrap_err().contains("APP_READ_ONLY"));
        // reads + the activation surface stay open
        assert!(!is_gated_command("search_products"));
        assert!(!is_gated_command("license_activate"));
        assert!(!is_gated_command("get_setting"));
        assert!(!is_gated_command("set_multiple_settings"));
    }

    #[test]
    fn full_license_passes_the_gate() {
        let db = test_db();
        set_license_setting(&db, "app_license_status", "full").unwrap();
        set_license_setting(&db, "app_license_key", "x.y").unwrap();
        let st = license_status(&db).unwrap();
        assert_eq!(st["status"].as_str().unwrap(), "full");
        assert_eq!(st["readonly"].as_bool().unwrap(), false);
        assert!(gate_command(&db, "process_sale").is_ok());
    }

    #[test]
    fn legacy_activated_maps_to_full() {
        let db = test_db();
        set_license_setting(&db, "app_license_status", "activated").unwrap();
        assert_eq!(license_status(&db).unwrap()["status"].as_str().unwrap(), "full");
        assert!(gate_command(&db, "save_product").is_ok());
    }

    #[test]
    fn trial_expiry_flips_to_expired_and_gates() {
        let db = test_db();
        set_license_setting(&db, "app_license_status", "trial").unwrap();
        set_license_setting(&db, "app_license_expiry", "2000-01-01").unwrap();
        let st = license_status(&db).unwrap();
        assert_eq!(st["status"].as_str().unwrap(), "expired");
        assert!(gate_command(&db, "process_sale").unwrap_err().contains("APP_READ_ONLY"));
    }

    #[test]
    fn revoked_status_gates() {
        let db = test_db();
        set_license_setting(&db, "app_license_status", "revoked").unwrap();
        let st = license_status(&db).unwrap();
        assert_eq!(st["status"].as_str().unwrap(), "revoked");
        assert!(gate_command(&db, "create_purchase").unwrap_err().contains("APP_READ_ONLY"));
    }

    #[test]
    fn boot_revalidation_wipes_tampered_status() {
        // DB-edited "full" status WITHOUT a valid signed key → back to none.
        let db = test_db();
        set_license_setting(&db, "app_license_status", "full").unwrap();
        revalidate_stored_license(&db);
        assert_eq!(license_status(&db).unwrap()["status"].as_str().unwrap(), "none");
    }

    #[test]
    fn boot_revalidation_keeps_legacy_activated() {
        let db = test_db();
        set_license_setting(&db, "app_license_status", "activated").unwrap();
        revalidate_stored_license(&db);
        assert_eq!(license_status(&db).unwrap()["status"].as_str().unwrap(), "full");
    }

    #[test]
    fn mutating_list_covers_the_business_surface() {
        for cmd in [
            "process_sale", "create_sale", "replace_sale", "save_product", "delete_product",
            "create_purchase", "add_expense", "open_cash_session", "add_cash_movement",
            "save_customer", "save_supplier", "save_employee", "factory_reset",
            "restore_database", "clear_transaction_history", "upload_product_to_scale",
        ] {
            assert!(is_gated_command(cmd), "'{}' must be gated", cmd);
        }
    }
}
