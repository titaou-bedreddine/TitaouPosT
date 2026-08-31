use crate::database::DbState;
use crate::models::{Supplier, SupplierPaymentInput, SupplierPaymentRow};
use rusqlite::Result;

pub fn list_suppliers(db: &DbState) -> Result<Vec<Supplier>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT id, name, contact_person, phone, email, address, rc, nif, nis, ai, qr_code, balance, notes, is_active, created_at
             FROM suppliers
             WHERE is_active = 1
             ORDER BY id DESC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Supplier {
                id: row.get(0)?,
                name: row.get(1)?,
                contact_person: row.get(2)?,
                phone: row.get(3)?,
                email: row.get(4)?,
                address: row.get(5)?,
                rc: row.get(6)?,
                nif: row.get(7)?,
                nis: row.get(8)?,
                ai: row.get(9)?,
                qr_code: row.get(10)?,
                balance: row.get(11)?,
                notes: row.get(12)?,
                is_active: row.get(13)?,
                created_at: row.get(14)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let list: Vec<Supplier> = rows.filter_map(|r| r.ok()).collect();
    Ok(list)
}

pub fn save_supplier(
    db: &DbState,
    name: &str,
    contact_person: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    address: Option<String>,
    rc: Option<String>,
    nif: Option<String>,
    nis: Option<String>,
    ai: Option<String>,
    notes: Option<String>,
    supplier_id: Option<i64>,
) -> Result<i64, String> {
    let conn = db.conn.lock().unwrap();

    if let Some(sid) = supplier_id {
        conn.execute(
            "UPDATE suppliers
             SET name = ?1, contact_person = ?2, phone = ?3, email = ?4, address = ?5, rc = ?6, nif = ?7, nis = ?8, ai = ?9, notes = ?10
             WHERE id = ?11",
            rusqlite::params![name, contact_person, phone, email, address, rc, nif, nis, ai, notes, sid],
        )
        .map_err(|e| e.to_string())?;
        Ok(sid)
    } else {
        let qr_code = format!("SUPP-{:04}", chrono::Local::now().timestamp_subsec_millis());
        conn.execute(
            "INSERT INTO suppliers (name, contact_person, phone, email, address, rc, nif, nis, ai, qr_code, balance, notes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 0, ?10)",
            rusqlite::params![name, contact_person, phone, email, address, rc, nif, nis, ai, qr_code, notes],
        )
        .map_err(|e| e.to_string())?;
        Ok(conn.last_insert_rowid())
    }
}

pub fn delete_supplier(db: &DbState, supplier_id: i64) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();
    conn.execute("UPDATE suppliers SET is_active = 0 WHERE id = ?1", [supplier_id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
/// Record a payment against a supplier's balance (money we owe them).
/// Cash payments land in the session's cash movements (money OUT of the
/// drawer) so the register and statistics reflect the payment.
pub fn record_supplier_debt_payment(db: &DbState, input: SupplierPaymentInput) -> Result<i64, String> {
    let mut conn = db.conn.lock().unwrap();
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    tx.execute(
        "INSERT INTO supplier_debt_payments (supplier_id, amount, payment_method, reference, session_id, user_id, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        rusqlite::params![
            input.supplier_id, input.amount, input.payment_method,
            input.reference, input.session_id, input.user_id, input.notes
        ],
    )
    .map_err(|e| e.to_string())?;

    let payment_id = tx.last_insert_rowid();

    // Never overpay past what we owe: clamp at 0.
    tx.execute(
        "UPDATE suppliers SET balance = MAX(0, balance - ?1) WHERE id = ?2",
        rusqlite::params![input.amount, input.supplier_id],
    )
    .map_err(|e| e.to_string())?;

    if input.payment_method == "cash" {
        if let Some(sid) = input.session_id {
            tx.execute(
                "INSERT INTO cash_movements (session_id, user_id, type, amount, reason, reference_type, reference_id)
                 VALUES (?1, ?2, 'supplier_debt_payment', ?3, ?4, 'supplier_payment', ?5)",
                rusqlite::params![
                    sid,
                    input.user_id,
                    -input.amount,
                    format!("Supplier Debt Payment / \u{062a}\u{0633}\u{062f}\u{064a}\u{062f} \u{062f}\u{064a}\u{0646} \u{0645}\u{0648}\u{0631}\u{062f} #{}", input.supplier_id),
                    payment_id
                ],
            )
            .map_err(|e| e.to_string())?;

            tx.execute(
                "UPDATE cash_sessions SET expected_cash = expected_cash - ?1 WHERE id = ?2",
                rusqlite::params![input.amount, sid],
            )
            .map_err(|e| e.to_string())?;
        }
    }

    tx.commit().map_err(|e| e.to_string())?;
    drop(conn);

    // Telegram "supplier payment" alert (fire-and-forget, lock released).
    crate::services::notifier_service::notify_if_enabled(
        db,
        "notify_supplier_payment",
        format!("\u{1f4b8} *Paiement Fournisseur*\n\u{1f4b0} Montant: *{} DZD*\n\u{1f4c2} Fournisseur #{}", input.amount, input.supplier_id),
    );

    Ok(payment_id)
}

/// Payment history for a supplier, newest first.
pub fn list_supplier_debt_payments(db: &DbState, supplier_id: i64) -> Result<Vec<SupplierPaymentRow>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT s.id, s.supplier_id, s.amount, s.payment_method, s.reference, s.created_at
             FROM supplier_debt_payments s
             WHERE s.supplier_id = ?1
             ORDER BY s.id DESC LIMIT 100",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([supplier_id], |row| {
            Ok(SupplierPaymentRow {
                id: row.get(0)?,
                supplier_id: row.get(1)?,
                amount: row.get(2)?,
                payment_method: row.get(3)?,
                reference: row.get(4)?,
                created_at: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;

    Ok(rows.filter_map(|r| r.ok()).collect())
}
