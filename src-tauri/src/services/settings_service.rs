use crate::database::DbState;
use rusqlite::Result;
use std::collections::HashMap;

pub fn get_all_settings(db: &DbState) -> Result<HashMap<String, String>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT key, value FROM app_settings")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)))
        .map_err(|e| e.to_string())?;

    let mut map = HashMap::new();
    for r in rows {
        if let Ok((k, v)) = r {
            map.insert(k, v);
        }
    }
    Ok(map)
}

pub fn set_setting(db: &DbState, key: &str, value: &str) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();
    conn.execute(
        "INSERT INTO app_settings (key, value, updated_at) VALUES (?1, ?2, CURRENT_TIMESTAMP)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = CURRENT_TIMESTAMP",
        rusqlite::params![key, value],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn set_multiple_settings(db: &DbState, settings: HashMap<String, String>) -> Result<(), String> {
    let mut conn = db.conn.lock().unwrap();
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    for (k, v) in settings {
        tx.execute(
            "INSERT INTO app_settings (key, value, updated_at) VALUES (?1, ?2, CURRENT_TIMESTAMP)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = CURRENT_TIMESTAMP",
            rusqlite::params![k, v],
        )
        .map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_hwid() -> String {
    "4e67912a-d4ee-49bc-9bd0-6dccc402a6e6".to_string()
}

pub fn verify_license(db: &DbState, code: &str) -> Result<bool, String> {
    if code.starts_with("LUM-") || code.starts_with("ACT-") || code.len() >= 12 {
        set_setting(db, "app_license_status", "activated")?;
        set_setting(db, "app_license_key", code)?;
        Ok(true)
    } else {
        Err("Invalid activation code format".to_string())
    }
}

pub fn factory_reset(db: &DbState, reset_type: &str) -> Result<(), String> {
    let mut conn = db.conn.lock().unwrap();
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    match reset_type {
        "transactions_only" => {
            tx.execute("DELETE FROM sales", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM sale_items", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM sale_payments", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM held_sales", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM cash_movements", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM cash_sessions", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM customer_debt_payments", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM supplier_debt_payments", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM purchases", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM purchase_items", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM expenses", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM inventory_movements", []).map_err(|e| e.to_string())?;
            tx.execute(
                "INSERT INTO cash_sessions (register_id, user_id, opening_amount, expected_cash, status) VALUES (1, 1, 0, 0, 'open')",
                [],
            ).map_err(|e| e.to_string())?;
        }
        "full_reset" => {
            tx.execute("DELETE FROM sales", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM sale_items", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM sale_payments", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM held_sales", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM cash_movements", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM cash_sessions", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM customer_debt_payments", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM supplier_debt_payments", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM purchases", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM purchase_items", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM expenses", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM inventory_movements", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM products", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM product_barcodes", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM customers WHERE id > 1", []).map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM suppliers", []).map_err(|e| e.to_string())?;
            tx.execute(
                "INSERT INTO cash_sessions (register_id, user_id, opening_amount, expected_cash, status) VALUES (1, 1, 0, 0, 'open')",
                [],
            ).map_err(|e| e.to_string())?;
        }
        _ => {}
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(())
}