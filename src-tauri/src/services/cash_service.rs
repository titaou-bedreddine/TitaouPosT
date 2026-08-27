use crate::database::DbState;
use crate::models::{CashMovement, CashSession};
use rusqlite::Result;

pub fn get_active_session(db: &DbState, user_id: i64) -> Result<Option<CashSession>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT id, register_id, user_id, opened_at, closed_at, opening_amount, expected_cash, actual_cash, difference, status, notes
             FROM cash_sessions
             WHERE user_id = ?1 AND status = 'open'
             ORDER BY id DESC LIMIT 1",
        )
        .map_err(|e| e.to_string())?;

    let session = stmt.query_row([user_id], |row| {
        Ok(CashSession {
            id: row.get(0)?,
            register_id: row.get(1)?,
            user_id: row.get(2)?,
            opened_at: row.get(3)?,
            closed_at: row.get(4)?,
            opening_amount: row.get(5)?,
            expected_cash: row.get(6)?,
            actual_cash: row.get(7)?,
            difference: row.get(8)?,
            status: row.get(9)?,
            notes: row.get(10)?,
        })
    });

    match session {
        Ok(s) => Ok(Some(s)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

pub fn open_session(db: &DbState, user_id: i64, register_id: i64, opening_amount: i64, notes: Option<String>) -> Result<CashSession, String> {
    let mut conn = db.conn.lock().unwrap();
    let tx = conn.transaction().map_err(|e| e.to_string())?;

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
        opened_at: chrono::Local::now().to_rfc3339(),
        closed_at: None,
        opening_amount,
        expected_cash: opening_amount,
        actual_cash: None,
        difference: None,
        status: "open".to_string(),
        notes,
    })
}

pub fn add_cash_movement(db: &DbState, session_id: i64, user_id: i64, movement_type: &str, amount: i64, reason: Option<String>) -> Result<(), String> {
    let mut conn = db.conn.lock().unwrap();
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    tx.execute(
        "INSERT INTO cash_movements (session_id, user_id, type, amount, reason)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![session_id, user_id, movement_type, amount, reason],
    )
    .map_err(|e| e.to_string())?;

    // Update expected cash
    let adjustment = if movement_type == "cash_in" || movement_type == "cash_deposit" {
        amount
    } else {
        -amount
    };

    tx.execute(
        "UPDATE cash_sessions SET expected_cash = expected_cash + ?1 WHERE id = ?2",
        rusqlite::params![adjustment, session_id],
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
            "SELECT id, session_id, user_id, type, amount, reason, created_at, notes
             FROM cash_movements
             WHERE session_id = ?1
             ORDER BY id DESC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([session_id], |row| {
            Ok(CashMovement {
                id: row.get(0)?,
                session_id: row.get(1)?,
                user_id: row.get(2)?,
                type_name: row.get(3)?,
                amount: row.get(4)?,
                reason: row.get(5)?,
                created_at: row.get(6)?,
                notes: row.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut list = Vec::new();
    for r in rows {
        if let Ok(m) = r {
            list.push(m);
        }
    }
    Ok(list)
}
