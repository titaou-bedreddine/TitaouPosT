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
    
    // Disable foreign keys temporarily during data wipe
    conn.execute("PRAGMA foreign_keys = OFF;", []).map_err(|e| e.to_string())?;

    let result = (|| -> Result<(), rusqlite::Error> {
        let tx = conn.transaction()?;

        match reset_type {
            "transactions_only" => {
                // Clear transaction tables
                let _ = tx.execute("DELETE FROM sale_payments", []);
                let _ = tx.execute("DELETE FROM sale_items", []);
                let _ = tx.execute("DELETE FROM sales", []);
                let _ = tx.execute("DELETE FROM held_sales", []);
                let _ = tx.execute("DELETE FROM cash_movements", []);
                let _ = tx.execute("DELETE FROM cash_sessions", []);
                let _ = tx.execute("DELETE FROM customer_debt_payments", []);
                let _ = tx.execute("DELETE FROM supplier_debt_payments", []);
                let _ = tx.execute("DELETE FROM purchase_items", []);
                let _ = tx.execute("DELETE FROM purchases", []);
                let _ = tx.execute("DELETE FROM expenses", []);
                let _ = tx.execute("DELETE FROM salary_advances", []);
                let _ = tx.execute("DELETE FROM payrolls", []);
                let _ = tx.execute("DELETE FROM inventory_movements", []);
                let _ = tx.execute("DELETE FROM scale_sync_logs", []);
                let _ = tx.execute("DELETE FROM product_price_history", []);
                let _ = tx.execute("DELETE FROM notification_queue", []);

                // Reset customer and supplier balances
                let _ = tx.execute("UPDATE customers SET balance = 0", []);
                let _ = tx.execute("UPDATE suppliers SET balance = 0", []);

                // Ensure default open cash session exists
                let _ = tx.execute(
                    "INSERT INTO cash_sessions (register_id, user_id, opening_amount, expected_cash, status) VALUES (1, 1, 0, 0, 'open')",
                    [],
                );
            }
            "full_reset" => {
                // Wipe everything back to clean state
                let _ = tx.execute("DELETE FROM sale_payments", []);
                let _ = tx.execute("DELETE FROM sale_items", []);
                let _ = tx.execute("DELETE FROM sales", []);
                let _ = tx.execute("DELETE FROM held_sales", []);
                let _ = tx.execute("DELETE FROM cash_movements", []);
                let _ = tx.execute("DELETE FROM cash_sessions", []);
                let _ = tx.execute("DELETE FROM customer_debt_payments", []);
                let _ = tx.execute("DELETE FROM supplier_debt_payments", []);
                let _ = tx.execute("DELETE FROM purchase_items", []);
                let _ = tx.execute("DELETE FROM purchases", []);
                let _ = tx.execute("DELETE FROM expenses", []);
                let _ = tx.execute("DELETE FROM salary_advances", []);
                let _ = tx.execute("DELETE FROM payrolls", []);
                let _ = tx.execute("DELETE FROM employees", []);
                let _ = tx.execute("DELETE FROM inventory_movements", []);
                let _ = tx.execute("DELETE FROM scale_sync_logs", []);
                let _ = tx.execute("DELETE FROM product_price_history", []);
                let _ = tx.execute("DELETE FROM notification_queue", []);
                let _ = tx.execute("DELETE FROM product_bundle_items", []);
                let _ = tx.execute("DELETE FROM product_barcodes", []);
                let _ = tx.execute("DELETE FROM products", []);
                let _ = tx.execute("DELETE FROM suppliers", []);
                let _ = tx.execute("DELETE FROM customers WHERE id > 1", []);
                let _ = tx.execute("UPDATE customers SET balance = 0 WHERE id = 1", []);
                let _ = tx.execute("DELETE FROM users WHERE id > 1", []);

                // Ensure default open cash session exists
                let _ = tx.execute(
                    "INSERT INTO cash_sessions (register_id, user_id, opening_amount, expected_cash, status) VALUES (1, 1, 0, 0, 'open')",
                    [],
                );
            }
            _ => {}
        }

        tx.commit()?;
        Ok(())
    })();

    // Re-enable foreign keys
    let _ = conn.execute("PRAGMA foreign_keys = ON;", []);

    result.map_err(|e| e.to_string())
}

pub fn backup_database(destination_path: &str) -> Result<String, String> {
    let source_path = crate::database::get_database_path();
    if !source_path.exists() {
        return Err("Source database file does not exist".to_string());
    }

    let dest = std::path::PathBuf::from(destination_path);
    if let Some(parent) = dest.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    std::fs::copy(&source_path, &dest).map_err(|e| format!("Failed to copy database: {}", e))?;
    Ok(format!("Backup successfully created at {}", destination_path))
}

pub fn restore_database(source_backup_path: &str) -> Result<String, String> {
    let backup_path = std::path::PathBuf::from(source_backup_path);
    if !backup_path.exists() {
        return Err("Backup file not found".to_string());
    }

    let target_path = crate::database::get_database_path();
    std::fs::copy(&backup_path, &target_path).map_err(|e| format!("Failed to restore database: {}", e))?;
    Ok("Database successfully restored! Please restart the application.".to_string())
}