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

    let session = stmt.query_row([], |row| {
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
        })
    });

    match session {
        Ok(mut s) => {
            // Calculate total sales and expenses for this session
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
        },
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
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

    tx.commit().map_err(|e| e.to_string())?;
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
            })
        })
        .map_err(|e| e.to_string())?;

    let list: Vec<CashSession> = rows.filter_map(|r| r.ok()).collect();
    Ok(list)
}