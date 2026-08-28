use crate::database::DbState;
use crate::models::Expense;
use rusqlite::Result;

pub fn add_expense(
    db: &DbState,
    category_id: i64,
    amount: i64,
    payment_method: &str,
    session_id: Option<i64>,
    user_id: i64,
    recipient: Option<String>,
    receipt_reference: Option<String>,
    notes: Option<String>,
) -> Result<String, String> {
    let mut conn = db.conn.lock().unwrap();
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let now = chrono::Local::now();
    let expense_number = format!("EXP-{}", now.format("%Y%m%d%H%M%S"));
    let today_date = now.format("%Y-%m-%d").to_string();

    tx.execute(
        "INSERT INTO expenses (expense_number, category_id, amount, payment_method, session_id, user_id, recipient, receipt_reference, date, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        rusqlite::params![
            expense_number, category_id, amount, payment_method,
            session_id, user_id, recipient, receipt_reference, today_date, notes
        ],
    )
    .map_err(|e| e.to_string())?;

    let expense_id = tx.last_insert_rowid();

    // If paid from active cash session drawer, deduct from cash movements
    if payment_method == "cash" {
        if let Some(sid) = session_id {
            tx.execute(
                "INSERT INTO cash_movements (session_id, user_id, type, amount, reason, reference_type, reference_id)
                 VALUES (?1, ?2, 'expense_payment', ?3, ?4, 'expense', ?5)",
                rusqlite::params![
                    sid, user_id, -amount,
                    format!("Expense Payment / دفع مصروف {}", expense_number), expense_id
                ],
            )
            .map_err(|e| e.to_string())?;

            tx.execute(
                "UPDATE cash_sessions SET expected_cash = expected_cash - ?1 WHERE id = ?2",
                rusqlite::params![amount, sid],
            )
            .map_err(|e| e.to_string())?;
        }
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(expense_number)
}

pub fn list_expenses(db: &DbState) -> Result<Vec<Expense>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT e.id, e.expense_number, e.category_id, ec.name_ar, e.amount,
                    e.payment_method, e.session_id, e.user_id, e.recipient, e.receipt_reference,
                    e.date, e.notes, e.created_at
             FROM expenses e
             LEFT JOIN expense_categories ec ON e.category_id = ec.id
             ORDER BY e.id DESC LIMIT 200",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Expense {
                id: row.get(0)?,
                expense_number: row.get(1)?,
                category_id: row.get(2)?,
                category_name: row.get(3)?,
                amount: row.get(4)?,
                payment_method: row.get(5)?,
                session_id: row.get(6)?,
                user_id: row.get(7)?,
                recipient: row.get(8)?,
                receipt_reference: row.get(9)?,
                date: row.get(10)?,
                notes: row.get(11)?,
                created_at: row.get(12)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let list: Vec<Expense> = rows.filter_map(|r| r.ok()).collect();
    Ok(list)
}

pub fn delete_expense(db: &DbState, expense_id: i64) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();
    conn.execute("DELETE FROM expenses WHERE id = ?1", [expense_id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

