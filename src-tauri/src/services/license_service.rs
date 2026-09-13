//! Cryptographic licensing (v0.5.34).
//!
//! Licenses are Ed25519 minisign signatures over a JSON payload bound to
//! the machine HWID. The shop owner holds the master SECRET key; the
//! PUBLIC key is compiled into this app, so clients can verify but never
//! forge licenses — even with full access to the bot token or database.
//!
//! Key string format (pasteable):  base64(payload_json) + "." + base64(signature_box)
//! .lic file format:               payload_json + "\n" + minisign signature box
//! Trial licenses carry an expiry date; full licenses none.

use crate::database::DbState;
use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine;
use serde_json::json;

/// Trusted public key (minisign, base64). Generated with the master
/// keypair; rotate by regenerating on the dev machine and replacing this
/// constant in the next build.
pub const LICENSE_PUBKEY: &str = "RWTLZ4gRtN0qIYbsbBblb34+DOspf2drcCUKrPkTj+tMRIWDQCvDmjMv";

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

/// Verify a pasted key or uploaded .lic against THIS machine and activate.
/// Returns (mode, expiry, shop).
pub fn verify_and_activate(db: &DbState, license_text: &str) -> Result<(String, Option<String>, String), String> {
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
    let mode = payload.mode.clone();
    if mode == "trial" {
        if let Some(exp) = &payload.expiry {
            if today() > *exp {
                return Err(format!("Trial license expired on {}", exp));
            }
        }
    }

    crate::services::settings_service::set_setting(db, "app_license_key", license_text)?;
    crate::services::settings_service::set_setting(db, "app_license_status", &mode)?;
    crate::services::settings_service::set_setting(
        db,
        "app_license_expiry",
        payload.expiry.as_deref().unwrap_or(""),
    )?;
    crate::services::settings_service::set_setting(db, "app_license_shop", &payload.shop)?;
    Ok((mode, payload.expiry.clone(), payload.shop.clone()))
}

/// Current license state for the UI (trial expiry enforced here).
pub fn license_status(db: &DbState) -> Result<serde_json::Value, String> {
    let settings = crate::services::settings_service::get_all_settings(db).unwrap_or_default();
    let mut status = settings
        .get("app_license_status")
        .cloned()
        .unwrap_or_else(|| "activated".into());
    let expiry = settings.get("app_license_expiry").cloned().unwrap_or_default();
    let shop = settings.get("app_license_shop").cloned().unwrap_or_default();
    if status == "trial" && !expiry.is_empty() && today() > expiry {
        status = "expired".into();
    }
    Ok(serde_json::json!({
        "status": status,
        "expiry": expiry,
        "shop": shop,
        "hwid": get_hwid(),
    }))
}

pub fn get_hwid() -> String {
    crate::services::settings_service::get_hwid()
}

