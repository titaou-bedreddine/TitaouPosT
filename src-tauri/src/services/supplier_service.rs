use crate::database::DbState;
use crate::models::Supplier;
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