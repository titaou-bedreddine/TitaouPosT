use crate::database::DbState;
use std::collections::HashMap;

/// Read Telegram bot config + notify switches from app_settings.
pub fn get_telegram_config(db: &DbState) -> Option<(String, String, HashMap<String, String>)> {
    let settings = crate::services::settings_service::get_all_settings(db).ok()?;
    let token = settings.get("telegram_bot_token")?.trim().to_string();
    let chat_id = settings.get("telegram_chat_id")?.trim().to_string();
    if token.is_empty() || chat_id.is_empty() {
        return None;
    }
    Some((token, chat_id, settings))
}

/// Fire-and-forget Telegram send. Blocking on the POS UI is unacceptable, so
/// callers spawn this on a background thread and ignore failures.
pub fn send_telegram_blocking(token: &str, chat_id: &str, text: &str) -> Result<(), String> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    let client = reqwest::blocking::Client::builder()
        .user_agent("TitaouPOS-Notifier")
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?;

    for attempt in 0..2 {
        let resp = client
            .post(&url)
            .form(&[("chat_id", chat_id), ("text", text), ("parse_mode", "Markdown")])
            .send();

        match resp {
            Ok(r) => {
                if r.status().is_success() {
                    return Ok(());
                }
                // If markdown parse error (status 400), fall back immediately to plain text without Markdown
                if r.status().as_u16() == 400 {
                    let plain_resp = client
                        .post(&url)
                        .form(&[("chat_id", chat_id), ("text", text)])
                        .send();
                    if let Ok(pr) = plain_resp {
                        if pr.status().is_success() {
                            return Ok(());
                        }
                    }
                }
            }
            Err(_) => {
                if attempt == 0 {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    continue;
                }
            }
        }
    }

    // Final fallback attempt: send plain text without parse_mode
    let final_resp = client
        .post(&url)
        .form(&[("chat_id", chat_id), ("text", text)])
        .send()
        .map_err(|e| format!("Telegram send failed: {}", e))?;

    if !final_resp.status().is_success() {
        let status = final_resp.status();
        let body = final_resp.text().unwrap_or_default();
        return Err(format!("Telegram error HTTP {}: {}", status, body));
    }

    Ok(())
}

/// True when the current LOCAL time falls inside one of the configured
/// quiet windows ("HH:MM-HH:MM" comma/semicolon separated, e.g.
/// "08:00-13:00,17:00-21:00"). Windows may wrap midnight.
fn in_quiet_hours(settings: &std::collections::HashMap<String, String>) -> bool {
    let raw = match settings.get("telegram_quiet_windows") {
        Some(v) => v.trim().to_string(),
        None => return false,
    };
    if raw.is_empty() {
        return false;
    }
    let now = chrono::Local::now();
    use chrono::Timelike;
    let now_mins = now.hour() as i32 * 60 + now.minute() as i32;
    let parse = |t: &str| -> Option<i32> {
        let mut it = t.trim().split(':');
        let h: i32 = it.next()?.trim().parse().ok()?;
        let m: i32 = it.next().unwrap_or("0").trim().parse().ok()?;
        Some(h * 60 + m)
    };
    for win in raw.split(|c| c == ',' || c == ';') {
        let mut parts = win.split('-');
        let (Some(st), Some(en)) = (parts.next(), parts.next()) else {
            continue;
        };
        let (Some(a), Some(b)) = (parse(st), parse(en)) else {
            continue;
        };
        if a <= b {
            if now_mins >= a && now_mins < b {
                return true;
            }
        } else if now_mins >= a || now_mins < b {
            // Window wraps midnight (e.g. 22:00-06:00).
            return true;
        }
    }
    false
}

/// Push a message if the given notify switch is enabled ("true") AND the
/// current time is not inside a quiet window.
pub fn notify_if_enabled(db: &DbState, switch_key: &str, text: String) {
    let cfg = match get_telegram_config(db) {
        Some(c) => c,
        None => return,
    };
    let (token, chat_id, settings) = cfg;
    let enabled = settings
        .get(switch_key)
        .map(|v| v == "true")
        .unwrap_or(false);
    if !enabled {
        return;
    }
    if settings.get("telegram_master_enabled").map(|v| v == "false").unwrap_or(false) {
        // Master kill-switch (the quick toggle): silent until re-enabled.
        return;
    }
    if in_quiet_hours(&settings) {
        return;
    }
    std::thread::spawn(move || {
        let _ = send_telegram_blocking(&token, &chat_id, &text);
    });
}

/// Recurring recap: called by the frontend scheduler; sends today's sales
/// summary since the last recap.
pub fn send_periodic_recap(db: &DbState) -> Result<String, String> {
    let (token, chat_id, settings) = get_telegram_config(db)
        .ok_or("Telegram bot token / chat ID not configured")?;

    let enabled = settings
        .get("notify_recap_enabled")
        .map(|v| v == "true")
        .unwrap_or(false);
    if !enabled {
        return Ok("Recap disabled in settings".to_string());
    }

    let conn = db.conn.lock().unwrap();

    // Totals since the last recap (falls back to today midnight).
    let since_expr = "COALESCE(
                   (SELECT value FROM app_settings WHERE key = 'last_recap_at'),
                   datetime('now', 'localtime', 'start of day'))";

    let total_sales: i64 = conn
        .query_row(
            &format!(
                "SELECT COALESCE(SUM(total_amount), 0)
                 FROM sales
                 WHERE status = 'completed'
                   AND created_at >= {}",
                since_expr
            ),
            [],
            |r| r.get(0),
        )
        .map_err(|e| e.to_string())?;

    let nb_sales: i64 = conn
        .query_row(
            &format!(
                "SELECT COUNT(*) FROM sales
                 WHERE status = 'completed'
                   AND created_at >= {}",
                since_expr
            ),
            [],
            |r| r.get(0),
        )
        .map_err(|e| e.to_string())?;

    let total_cash: i64 = conn
        .query_row(
            &format!(
                "SELECT COALESCE(SUM(amount), 0) FROM cash_movements
                 WHERE type = 'cash_sale'
                   AND created_at >= {}",
                since_expr
            ),
            [],
            |r| r.get(0),
        )
        .map_err(|e| e.to_string())?;

    let total_expenses: i64 = conn
        .query_row(
            &format!(
                "SELECT COALESCE(SUM(amount), 0) FROM expenses
                 WHERE created_at >= {}",
                since_expr
            ),
            [],
            |r| r.get(0),
        )
        .map_err(|e| e.to_string())?;

    drop(conn);

    let now = chrono::Local::now().format("%d/%m %H:%M").to_string();
    let text = format!(
        "*📊 TitaouPOS Recap — {}*\n🧾 Sales: {} → *{} DZD*\n💵 Cash in: {} DZD\n💸 Expenses: {} DZD\n💰 Net: {} DZD",
        now,
        nb_sales,
        total_sales,
        total_cash,
        total_expenses,
        total_sales - total_expenses
    );

    send_telegram_blocking(&token, &chat_id, &text)?;

    // Stamp the recap time so the next window starts from here.
    crate::services::settings_service::set_setting(
        db,
        "last_recap_at",
        &chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    )?;

    Ok("Recap sent".to_string())
}

/// Localized Telegram strings keyed by the UI language setting.
pub fn tr(lang: &str, msgs: (String, String, String)) -> String {
    match lang {
        "ar" => msgs.1,
        "fr" => msgs.2,
        _ => msgs.0,
    }
}

/// Current UI language for notifications ("en" default).
pub fn ui_language(db: &DbState) -> String {
    crate::services::settings_service::get_all_settings(db)
        .ok()
        .and_then(|s| s.get("ui_language").cloned())
        .unwrap_or_else(|| "en".to_string())
}

/// Resolve the acting user's display name for notification attribution.
/// `user_id` comes from the frontend (the signed-in cashier / operator); when
/// absent or unknown, fall back to a neutral "System" label.
pub fn actor_label(db: &DbState, user_id: Option<i64>) -> String {
    let Some(uid) = user_id else {
        return "System".to_string();
    };
    let conn = db.conn.lock().unwrap();
    conn.query_row(
        "SELECT COALESCE(display_name, username) FROM users WHERE id = ?1",
        [uid],
        |r| r.get::<_, String>(0),
    )
    .unwrap_or_else(|_| format!("User #{}", uid))
}
