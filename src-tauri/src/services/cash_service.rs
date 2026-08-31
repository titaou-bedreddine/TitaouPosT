use crate::database::DbState;
use crate::models::{CashMovement, CashSession};
use rusqlite::Result;

pub fn get_active_session(db: &DbState, _user_id: i64) -> Result<Option<CashSession>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT cs.id, cs.register_id, cs.user_id, u.display_name, cs.opened_at, cs.closed_at,
                    cs.opening_amount, cs.expected_cash, cs.actual_cash, cs.difference, cs.status, cs.notes
             FROM cash_sessions cs
             LEFT JOIN users u ON cs.user_id = u.id
             WHERE cs.status = 'open'
             ORDER BY cs.id DESC LIMIT 1",
        )
        .map_err(|e| e.to_string())?;

    let session = match stmt.query_row([], |row| {
        Ok(CashSession {
            id: row.get(0)?,
            register_id: row.get(1)?,
            user_id: row.get(2)?,
            user_name: row.get(3)?,
            opened_at: row.get(4)?,
            closed_at: row.get(5)?,
            opening_amount: row.get(6)?,
            expected_cash: row.get(7)?,
            actual_cash: row.get(8)?,
            difference: row.get(9)?,
            total_sales: Some(0),
            total_expenses: Some(0),
            current_balance: Some(row.get(7)?),
            status: row.get(10)?,
            notes: row.get(11)?,
            is_stale: None,
        })
    }) {
        Ok(s) => Some(s),
        Err(rusqlite::Error::QueryReturnedNoRows) => None,
        Err(e) => return Err(e.to_string()),
    };
    drop(stmt);
    drop(conn);

    match session {
        Some(mut s) => {
            // A session left open from a previous calendar day is stale: the
            // POS day ends at midnight, so it auto-closes here (cash counted
            // as expected) and the caller opens today's fresh session.
            // Closing the app must NEVER close a session. opened_at is
            // stored as "YYYY-MM-DD HH:MM:SS", not RFC3339 — parse both.
            let opened_date = chrono::NaiveDateTime::parse_from_str(
                &s.opened_at,
                "%Y-%m-%d %H:%M:%S",
            )
            .map(|d| d.date())
            .or_else(|_| {
                chrono::DateTime::parse_from_rfc3339(&s.opened_at)
                    .map(|d| d.with_timezone(&chrono::Local).date_naive())
            })
            .unwrap_or_else(|_| chrono::Local::now().date_naive());
            let is_stale = opened_date < chrono::Local::now().date_naive();

            if is_stale {
                // Previous-day session: close it now (midnight rollover);
                // the caller sees no active session and opens a fresh one.
                let conn = db.conn.lock().unwrap();
                let _ = conn.execute(
                    "UPDATE cash_sessions
                     SET closed_at = CURRENT_TIMESTAMP, actual_cash = expected_cash,
                         difference = 0, status = 'closed',
                         notes = COALESCE(notes, '') || ' | Auto-closed at midnight'
                     WHERE id = ?1 AND status = 'open'",
                    [s.id],
                );
                return Ok(None);
            }
            s.is_stale = Some(false);

            // Calculate total sales and expenses for this session
            let conn = db.conn.lock().unwrap();
            let sales_sum: i64 = conn.query_row(
                "SELECT COALESCE(SUM(amount), 0) FROM cash_movements WHERE session_id = ?1 AND type = 'cash_sale'",
                [s.id],
                |r| r.get(0),
            ).unwrap_or(0);

            let exp_sum: i64 = conn.query_row(
                "SELECT COALESCE(SUM(amount), 0) FROM cash_movements WHERE session_id = ?1 AND (type = 'expense_payment' OR type = 'cash_out')",
                [s.id],
                |r| r.get(0),
            ).unwrap_or(0);

            s.total_sales = Some(sales_sum);
            s.total_expenses = Some(exp_sum.abs());
            s.current_balance = Some(s.expected_cash);
            Ok(Some(s))
        }
        None => Ok(None),
    }
}

pub fn open_session(db: &DbState, user_id: i64, register_id: i64, opening_amount: i64, notes: Option<String>) -> Result<CashSession, String> {
    let mut conn = db.conn.lock().unwrap();
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    // Close any previous open sessions
    let _ = tx.execute("UPDATE cash_sessions SET status = 'closed', closed_at = CURRENT_TIMESTAMP WHERE status = 'open'", []);

    tx.execute(
        "INSERT INTO cash_sessions (register_id, user_id, opening_amount, expected_cash, status, notes)
         VALUES (?1, ?2, ?3, ?3, 'open', ?4)",
        rusqlite::params![register_id, user_id, opening_amount, notes],
    )
    .map_err(|e| e.to_string())?;

    let session_id = tx.last_insert_rowid();

    tx.execute(
        "INSERT INTO cash_movements (session_id, user_id, type, amount, reason, notes)
         VALUES (?1, ?2, 'opening_balance', ?3, 'Startup Cash / رصيد افتتاحي', ?4)",
        rusqlite::params![session_id, user_id, opening_amount, notes],
    )
    .map_err(|e| e.to_string())?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok(CashSession {
        id: session_id,
        register_id,
        user_id,
        user_name: Some("Cashier".to_string()),
        opened_at: chrono::Local::now().to_rfc3339(),
        closed_at: None,
        opening_amount,
        expected_cash: opening_amount,
        actual_cash: None,
        difference: None,
        total_sales: Some(0),
        total_expenses: Some(0),
        current_balance: Some(opening_amount),
        status: "open".to_string(),
        notes,
        is_stale: Some(false),
    })
}

pub fn add_cash_movement(db: &DbState, session_id: i64, user_id: i64, movement_type: &str, amount: i64, reason: Option<String>) -> Result<(), String> {
    let mut conn = db.conn.lock().unwrap();
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let signed_amount = if movement_type == "cash_in" || movement_type == "opening_balance" || movement_type == "customer_debt_payment" {
        amount.abs()
    } else {
        -amount.abs()
    };

    let reason_for_log = reason.clone();
    tx.execute(
        "INSERT INTO cash_movements (session_id, user_id, type, amount, reason)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![session_id, user_id, movement_type, signed_amount, reason],
    )
    .map_err(|e| e.to_string())?;

    tx.execute(
        "UPDATE cash_sessions SET expected_cash = expected_cash + ?1 WHERE id = ?2",
        rusqlite::params![signed_amount, session_id],
    )
    .map_err(|e| e.to_string())?;

    // A withdrawal from the drawer (POS drawer modal or register page) is
    // money leaving the shop: book it as an expense too, so the Expenses
    // page and dashboard statistics include it. Without this, cash_out
    // only showed on the register page.
    if movement_type == "cash_out" && amount > 0 {
        let now = chrono::Local::now();
        let expense_number = format!("EXP-{}", now.format("%Y%m%d%H%M%S"));
        tx.execute(
            "INSERT INTO expenses (expense_number, category_id, amount, payment_method, session_id, user_id, recipient, receipt_reference, date, notes)
             VALUES (?1, 6, ?2, 'cash', ?3, ?4, 'Cash Register', NULL, ?5, ?6)",
            rusqlite::params![
                expense_number,
                amount,
                session_id,
                user_id,
                now.format("%Y-%m-%d").to_string(),
                reason.unwrap_or_else(|| "Cash withdrawal from drawer / سحب نقدي من الصندوق".to_string()),
            ],
        )
        .map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;
    drop(conn);

    // Telegram alerts for drawer money movement (fire-and-forget).
    match movement_type {
        "cash_in" => crate::services::notifier_service::notify_if_enabled(
            db,
            "notify_cash_in",
            format!("\u{1f4b0} *Entree Caisse*\nMontant: *{} DZD*\nMotif: {}", amount, reason_for_log.as_deref().unwrap_or("-")),
        ),
        "cash_out" => crate::services::notifier_service::notify_if_enabled(
            db,
            "notify_cash_out",
            format!("\u{1f4b8} *Sortie Caisse*\nMontant: *{} DZD*\nMotif: {}", amount, reason_for_log.as_deref().unwrap_or("-")),
        ),
        "opening_balance" => crate::services::notifier_service::notify_if_enabled(
            db,
            "notify_opening_cash",
            format!("\u{1f3e6} *Ouverture Caisse*\nFond de caisse: *{} DZD*", amount),
        ),
        _ => {}
    }

    Ok(())
}

pub fn close_session(db: &DbState, session_id: i64, actual_cash: i64, notes: Option<String>) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();
    
    let expected: i64 = conn
        .query_row(
            "SELECT expected_cash FROM cash_sessions WHERE id = ?1",
            [session_id],
            |r| r.get(0),
        )
        .map_err(|e| e.to_string())?;

    let difference = actual_cash - expected;

    conn.execute(
        "UPDATE cash_sessions
         SET closed_at = CURRENT_TIMESTAMP, actual_cash = ?1, difference = ?2, status = 'closed', notes = ?3
         WHERE id = ?4",
        rusqlite::params![actual_cash, difference, notes, session_id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn list_movements(db: &DbState, session_id: i64) -> Result<Vec<CashMovement>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT cm.id, cm.session_id, cm.user_id, u.display_name, cm.type, cm.amount, cm.reason, cm.created_at, cm.notes
             FROM cash_movements cm
             LEFT JOIN users u ON cm.user_id = u.id
             WHERE cm.session_id = ?1
             ORDER BY cm.id DESC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([session_id], |row| {
            Ok(CashMovement {
                id: row.get(0)?,
                session_id: row.get(1)?,
                user_id: row.get(2)?,
                user_name: row.get(3)?,
                type_name: row.get(4)?,
                amount: row.get(5)?,
                reason: row.get(6)?,
                created_at: row.get(7)?,
                notes: row.get(8)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let list: Vec<CashMovement> = rows.filter_map(|r| r.ok()).collect();
    Ok(list)
}

pub fn list_session_history(db: &DbState) -> Result<Vec<CashSession>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT cs.id, cs.register_id, cs.user_id, u.display_name, cs.opened_at, cs.closed_at,
                    cs.opening_amount, cs.expected_cash, cs.actual_cash, cs.difference, cs.status, cs.notes
             FROM cash_sessions cs
             LEFT JOIN users u ON cs.user_id = u.id
             ORDER BY cs.id DESC LIMIT 100",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(CashSession {
                id: row.get(0)?,
                register_id: row.get(1)?,
                user_id: row.get(2)?,
                user_name: row.get(3)?,
                opened_at: row.get(4)?,
                closed_at: row.get(5)?,
                opening_amount: row.get(6)?,
                expected_cash: row.get(7)?,
                actual_cash: row.get(8)?,
                difference: row.get(9)?,
                total_sales: None,
                total_expenses: None,
                current_balance: Some(row.get(7)?),
                status: row.get(10)?,
                notes: row.get(11)?,
                is_stale: None,
            })
        })
        .map_err(|e| e.to_string())?;

    let list: Vec<CashSession> = rows.filter_map(|r| r.ok()).collect();
    Ok(list)
}
#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn test_db_with_open_session(opened_at: &str) -> DbState {
        let conn = Connection::open_in_memory().unwrap();
        let state = DbState { conn: std::sync::Mutex::new(conn) };
        state.run_migrations().unwrap();
        state.seed_default_admin().unwrap();
        {
            let conn = state.conn.lock().unwrap();
            let _ = conn.execute(
                "INSERT INTO cash_sessions (register_id, user_id, opening_amount, expected_cash, status, opened_at)
                 VALUES (1, 1, 500, 500, 'open', ?1)",
                [opened_at],
            );
        }
        state
    }

    // A same-day session MUST be returned open: closing/reopening the app
    // never closes the register. Regression: the app used to force-close
    // still-valid sessions on every restart.
    #[test]
    fn same_day_session_survives() {
        // Dynamic "this morning" date — a hardcoded one goes stale past midnight.
        let this_morning = chrono::Local::now()
            .format("%Y-%m-%d 09:00:00")
            .to_string();
        let db = test_db_with_open_session(&this_morning);
        let s = get_active_session(&db, 1).unwrap().expect("session must be open");
        assert_eq!(s.status, "open");
        assert_eq!(s.is_stale, Some(false));
    }

    // A previous-day session is auto-closed at the midnight rollover so the
    // cashier starts a fresh one.
    #[test]
    fn previous_day_session_auto_closes() {
        // Dynamic "yesterday evening" so the test never expires.
        let yesterday = (chrono::Local::now() - chrono::Duration::days(1))
            .format("%Y-%m-%d 23:00:00")
            .to_string();
        let db = test_db_with_open_session(&yesterday);
        let s = get_active_session(&db, 1).unwrap();
        assert!(s.is_none(), "stale session must be closed (None)");
        let conn = db.conn.lock().unwrap();
        let status: String = conn
            .query_row("SELECT status FROM cash_sessions ORDER BY id DESC LIMIT 1", [], |r| r.get(0))
            .unwrap();
        assert_eq!(status, "closed");
    }

    // No open session at all returns None (not an error) so the UI can offer
    // "Open New Session".
    #[test]
    fn no_open_session_returns_none() {
        let conn = Connection::open_in_memory().unwrap();
        let state = DbState { conn: std::sync::Mutex::new(conn) };
        state.run_migrations().unwrap();
        state.seed_default_admin().unwrap();
        assert!(get_active_session(&state, 1).unwrap().is_none());
    }
}
