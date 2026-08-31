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
    // Real machine fingerprint: volume serial + hostname hash. Stable
    // across reboots, unique per PC (the licensing unit).
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        let out = std::process::Command::new("cmd")
            .args(["/C", "vol C:"])
            .creation_flags(0x08000000)
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
            .unwrap_or_default();
        let serial = out
            .lines()
            .find(|l| l.contains("Serial Number"))
            .and_then(|l| l.split(':').nth(1))
            .map(|s| s.trim().replace('-', ""))
            .unwrap_or_default();
        let machine = std::env::var("COMPUTERNAME").unwrap_or_default();
        // Short stable hash of serial+machine.
        let combined = format!("{}:{}", serial, machine);
        let mut hash: u64 = 5381;
        for b in combined.bytes() {
            hash = hash.wrapping_mul(33).wrapping_add(b as u64);
        }
        format!("HW-{hash:016X}")
    }
    #[cfg(not(windows))]
    {
        format!("HW-{:016X}", 0)
    }
}

/// Offline grace: a manual code still activates (format-checked), for
/// machines without internet.
pub fn verify_license(db: &DbState, code: &str) -> Result<bool, String> {
    if code.starts_with("LUM-") || code.starts_with("ACT-") || code.len() >= 12 {
        set_setting(db, "app_license_status", "activated")?;
        set_setting(db, "app_license_key", code)?;
        Ok(true)
    } else {
        Err("Invalid activation code format".to_string())
    }
}

/// Online activation against a GitHub-hosted license registry: the seller
/// pushes a JSON file named HWID.json to the licenses folder of a public
/// repo; the app fetches it on activation.
pub fn activate_online_github(
    db: &DbState,
    hwid: &str,
    github_user: &str,
    github_repo: &str,
) -> Result<bool, String> {
    let url = format!(
        "https://raw.githubusercontent.com/{}/{}/main/licenses/{}.json",
        github_user, github_repo, hwid
    );
    let response = reqwest::blocking::Client::builder()
        .user_agent("TitaouPOS-Activator")
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?
        .get(&url)
        .send()
        .map_err(|e| format!("Activation server unreachable: {}", e))?;

    if !response.status().is_success() {
        return Err("This machine has no license on the server. Contact the developer.".to_string());
    }

    let body: serde_json::Value = response
        .json()
        .map_err(|e| format!("Bad license data: {}", e))?;
    let licensed = body.get("licensed").and_then(|v| v.as_bool()).unwrap_or(false);
    if !licensed {
        return Err("License record is not active".to_string());
    }

    let key = body
        .get("license_key")
        .and_then(|v| v.as_str())
        .unwrap_or("GITHUB")
        .to_string();

    set_setting(db, "app_license_status", "activated")?;
    set_setting(db, "app_license_key", &key)?;
    Ok(true)
}

pub fn factory_reset(db: &DbState, reset_type: &str) -> Result<(), String> {
    let mut conn = db.conn.lock().unwrap();
    
    // Disable foreign keys temporarily during data wipe
    conn.execute("PRAGMA foreign_keys = OFF;", []).map_err(|e| e.to_string())?;

    let result = (|| -> Result<(), rusqlite::Error> {
        let tx = conn.transaction()?;

        match reset_type {
            "products_only" => {
                // Clear all products and related child data
                let _ = tx.execute("DELETE FROM sale_items", []);
                let _ = tx.execute("DELETE FROM purchase_items", []);
                let _ = tx.execute("DELETE FROM inventory_movements", []);
                let _ = tx.execute("DELETE FROM scale_sync_logs", []);
                let _ = tx.execute("DELETE FROM product_price_history", []);
                let _ = tx.execute("DELETE FROM product_bundle_items", []);
                let _ = tx.execute("DELETE FROM product_barcodes", []);
                let _ = tx.execute("DELETE FROM products", []);
            }
            "categories_only" => {
                // Reset categories back to default
                let _ = tx.execute("DELETE FROM categories", []);
                let _ = tx.execute(
                    "INSERT INTO categories (id, name_ar, name_fr, name_en, color, is_active) VALUES (1, 'افتراضي (Default)', 'Général / Default', 'Default', '#0284c7', 1)",
                    [],
                );
                let _ = tx.execute("UPDATE products SET category_id = 1", []);
            }
            "units_only" => {
                // Reset units back to standard system units
                let _ = tx.execute("DELETE FROM units", []);
                let _ = tx.execute(
                    "INSERT INTO units (id, name, short_name, allow_decimals) VALUES
                     (1, 'Piece / Pièce / قطعة', 'pcs', 0),
                     (2, 'Kilogram / Kilogramme / كيلوغرام', 'kg', 1),
                     (3, 'Liter / Litre / لتر', 'L', 1),
                     (4, 'Pack / Paquet / علبة', 'pck', 0),
                     (5, 'Box / Carton / كرتون', 'box', 0)",
                    [],
                );
                let _ = tx.execute("UPDATE products SET unit_id = 1", []);
            }
            "customers_only" => {
                // Reset customers and customer debts
                let _ = tx.execute("DELETE FROM customer_debt_payments", []);
                let _ = tx.execute("DELETE FROM customers WHERE id > 1", []);
                let _ = tx.execute("UPDATE customers SET balance = 0 WHERE id = 1", []);
            }
            "suppliers_only" => {
                // Reset suppliers and supplier debts
                let _ = tx.execute("DELETE FROM supplier_debt_payments", []);
                let _ = tx.execute("DELETE FROM purchase_items", []);
                let _ = tx.execute("DELETE FROM purchases", []);
                let _ = tx.execute("DELETE FROM suppliers", []);
            }
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

                // Reset categories to default
                let _ = tx.execute("DELETE FROM categories", []);
                let _ = tx.execute(
                    "INSERT INTO categories (id, name_ar, name_fr, name_en, color, is_active) VALUES (1, 'افتراضي (Default)', 'Général / Default', 'Default', '#0284c7', 1)",
                    [],
                );

                // Reset units to standard
                let _ = tx.execute("DELETE FROM units", []);
                let _ = tx.execute(
                    "INSERT INTO units (id, name, short_name, allow_decimals) VALUES
                     (1, 'Piece / Pièce / قطعة', 'pcs', 0),
                     (2, 'Kilogram / Kilogramme / كيلوغرام', 'kg', 1),
                     (3, 'Liter / Litre / لتر', 'L', 1),
                     (4, 'Pack / Paquet / علبة', 'pck', 0),
                     (5, 'Box / Carton / كرتون', 'box', 0)",
                    [],
                );

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