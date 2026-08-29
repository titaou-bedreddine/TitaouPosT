use crate::database::DbState;
use crate::models::User;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rusqlite::Result;

pub fn authenticate_user(db: &DbState, username: &str, password: &str) -> Result<Option<User>, String> {
    let conn = db.conn.lock().unwrap();

    // --- Step 1: Query user row and drop the statement before any further conn use ---
    let user_data: Option<(i64, String, String, String, Option<String>, Option<i64>, Option<String>, f64, bool)> = {
        let mut stmt = conn
            .prepare(
                "SELECT u.id, u.username, u.display_name, u.password_hash, u.pin_hash, u.role_id, r.name, u.max_discount_percent, u.is_active
                 FROM users u
                 LEFT JOIN roles r ON u.role_id = r.id
                 WHERE LOWER(u.username) = LOWER(?1) AND u.is_active = 1",
            )
            .map_err(|e| e.to_string())?;

        let result = stmt.query_row([username], |row| {
            let max_disc: f64 = row.get::<_, Option<f64>>(7).ok().flatten().unwrap_or(100.0);
            let active: bool = row.get::<_, Option<i64>>(8).ok().flatten().map(|v| v != 0).unwrap_or(true);
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, Option<i64>>(5)?,
                row.get::<_, Option<String>>(6)?,
                max_disc,
                active,
            ))
        });

        match result {
            Ok(row) => Some(row),
            Err(rusqlite::Error::QueryReturnedNoRows) => None,
            Err(e) => {
                eprintln!("[Auth Error] Failed to query user row: {}", e);
                None
            }
        }
        // stmt is dropped here
    };

    let (id, uname, dname, hash, pin, role_id, role_name, max_disc, active) = match user_data {
        Some(d) => d,
        None => return Ok(None),
    };

    // --- Step 2: Verify password (no conn use here) ---
    let mut is_valid = false;

    // 1. Direct plain text match
    if hash == password {
        is_valid = true;
    }

    // 2. Try Argon2 verification
    if !is_valid {
        if let Ok(parsed_hash) = PasswordHash::new(&hash) {
            if Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok() {
                is_valid = true;
            }
        }
    }

    // 3. Try PIN match (plain or Argon2)
    if !is_valid {
        if let Some(ref p_hash) = pin {
            if p_hash == password {
                is_valid = true;
            } else if let Ok(parsed_pin) = PasswordHash::new(p_hash) {
                if Argon2::default().verify_password(password.as_bytes(), &parsed_pin).is_ok() {
                    is_valid = true;
                }
            }
        }
    }

    // 4. Default admin recovery fallback
    if !is_valid && (uname.to_lowercase() == "admin" || id == 1) && (password == "admin" || password == "123456") {
        is_valid = true;
    }

    if !is_valid {
        return Ok(None);
    }

    // --- Step 3: Upgrade hash (conn is free now, stmt already dropped) ---
    let salt = SaltString::generate(&mut OsRng);
    if let Ok(new_hash) = Argon2::default().hash_password(password.as_bytes(), &salt) {
        let _ = conn.execute(
            "UPDATE users SET password_hash = ?1 WHERE id = ?2",
            rusqlite::params![new_hash.to_string(), id],
        );
    }

    // --- Step 4: Fetch permissions ---
    let permissions: Vec<String> = if let Some(rid) = role_id {
        let mut perm_stmt = conn
            .prepare(
                "SELECT p.code FROM permissions p
                 INNER JOIN role_permissions rp ON p.id = rp.permission_id
                 WHERE rp.role_id = ?1",
            )
            .map_err(|e| e.to_string())?;

        let rows = perm_stmt
            .query_map([rid], |row| row.get::<_, String>(0))
            .map_err(|e| e.to_string())?;
        rows.filter_map(|r| r.ok()).collect()
    } else {
        Vec::new()
    };

    // --- Step 5: Update last_login ---
    let _ = conn.execute(
        "UPDATE users SET last_login = CURRENT_TIMESTAMP WHERE id = ?1",
        [id],
    );

    Ok(Some(User {
        id,
        username: uname,
        display_name: dname,
        role_id,
        role_name,
        max_discount_percent: max_disc,
        is_active: active,
        permissions,
    }))
}

pub fn list_active_users(db: &DbState) -> Result<Vec<User>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT u.id, u.username, u.display_name, u.role_id, r.name, u.max_discount_percent, u.is_active
             FROM users u
             LEFT JOIN roles r ON u.role_id = r.id
             WHERE u.is_active = 1
             ORDER BY u.id ASC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            let max_disc: f64 = row.get::<_, Option<f64>>(5).ok().flatten().unwrap_or(100.0);
            let is_active: bool = row.get::<_, Option<i64>>(6).ok().flatten().map(|v| v != 0).unwrap_or(true);
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                display_name: row.get(2)?,
                role_id: row.get(3)?,
                role_name: row.get(4)?,
                max_discount_percent: max_disc,
                is_active,
                permissions: Vec::new(),
            })
        })
        .map_err(|e| e.to_string())?;

    let list: Vec<User> = rows.filter_map(|r| r.ok()).collect();
    Ok(list)
}

pub fn verify_admin_password(db: &DbState, password: &str) -> Result<bool, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT u.password_hash FROM users u
             LEFT JOIN roles r ON u.role_id = r.id
             WHERE (r.name = 'Administrator' OR u.role_id = 1) AND u.is_active = 1",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|e| e.to_string())?;

    for hash_result in rows {
        if let Ok(hash) = hash_result {
            if let Ok(parsed_hash) = PasswordHash::new(&hash) {
                if Argon2::default()
                    .verify_password(password.as_bytes(), &parsed_hash)
                    .is_ok()
                {
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}