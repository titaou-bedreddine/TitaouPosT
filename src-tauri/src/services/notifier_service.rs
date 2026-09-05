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
/// Retries transient (network/5xx) failures once; parse errors fall back to
/// plain text; every failure is logged so drops are diagnosable, and a
/// failure here never affects the caller.
pub fn send_telegram_blocking(token: &str, chat_id: &str, text: &str) -> Result<(), String> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    let client = reqwest::blocking::Client::builder()
        .user_agent("TitaouPOS-Notifier")
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?;

    let send_plain = |c: &reqwest::blocking::Client, txt: &str| -> Result<bool, String> {
        let resp = c
            .post(&url)
            .form(&[("chat_id", chat_id), ("text", txt)])
            .send()
            .map_err(|e| format!("network: {}", e))?;
        let ok = resp.status().is_success();
        if !ok {
            let status = resp.status();
            let body = resp.text().unwrap_or_default();
            eprintln!("[telegram] plain send failed: HTTP {} — {}", status, body);
        }
        Ok(ok)
    };

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
                let status = r.status();
                let body = r.text().unwrap_or_default();
                eprintln!("[telegram] send attempt {} failed: HTTP {} — {}", attempt + 1, status, body);
                // Markdown parse error → immediate plain-text fallback.
                if status.as_u16() == 400 {
                    if send_plain(&client, text).unwrap_or(false) {
                        return Ok(());
                    }
                }
                // 5xx/transient → retry once after a short pause.
                if status.is_server_error() && attempt == 0 {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    continue;
                }
            }
            Err(e) => {
                eprintln!("[telegram] network error (attempt {}): {}", attempt + 1, e);
                if attempt == 0 {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    continue;
                }
            }
        }
    }

    // Final fallback attempt: send plain text without parse_mode.
    if send_plain(&client, text).unwrap_or(false) {
        return Ok(());
    }
    Err("Telegram send failed after retries (see log)".to_string())
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
        if let Err(e) = send_telegram_blocking(&token, &chat_id, &text) {
            eprintln!("[telegram] notification dropped: {}", e);
        }
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

/// Same lookup, but usable INSIDE an open transaction: takes any executor
/// that derefs to a rusqlite connection (the caller's &Transaction) so it
/// never re-locks the shared DbState mutex.
pub fn actor_label_from_conn<T: std::ops::Deref<Target = rusqlite::Connection>>(
    conn: &T,
    user_id: Option<i64>,
) -> String {
    let Some(uid) = user_id else {
        return "System".to_string();
    };
    conn.query_row(
        "SELECT COALESCE(display_name, username) FROM users WHERE id = ?1",
        [uid],
        |r| r.get::<_, String>(0),
    )
    .unwrap_or_else(|_| format!("User #{}", uid))
}

// ---------------------------------------------------------------------------
// In-app notification feed (payroll reminders and future admin-action
// alerts). Rows persist in app_notification_log so reminders survive
// restarts and are listed on the Notifications page.
// ---------------------------------------------------------------------------

/// Append one row to the persistent in-app notification feed.
/// Failures are logged and swallowed: alerting must never break the POS.
pub fn push_inapp_notification(
    db: &DbState,
    ntype: &str,
    title: &str,
    message: &str,
    related_id: Option<i64>,
) {
    let conn = db.conn.lock().unwrap();
    let res = conn.execute(
        "CREATE TABLE IF NOT EXISTS app_notification_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            type TEXT NOT NULL,
            title TEXT NOT NULL,
            message TEXT NOT NULL,
            related_id INTEGER,
            is_dismissed INTEGER DEFAULT 0,
            created_at TEXT DEFAULT (datetime('now','localtime'))
        )",
        [],
    );
    if let Err(e) = res {
        eprintln!("[inapp-notify] could not ensure table: {}", e);
        return;
    }
    if let Err(e) = conn.execute(
        "INSERT INTO app_notification_log (type, title, message, related_id) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![ntype, title, message, related_id],
    ) {
        eprintln!("[inapp-notify] insert failed: {}", e);
    }
}
