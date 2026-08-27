use crate::database::DbState;
use crate::models::User;
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use rusqlite::Result;

pub fn authenticate_user(db: &DbState, username: &str, password: &str) -> Result<Option<User>, String> {
    let conn = db.conn.lock().unwrap();

    let mut stmt = conn
        .prepare(
            "SELECT u.id, u.username, u.display_name, u.password_hash, u.role_id, r.name, u.max_discount_percent, u.is_active
             FROM users u
             LEFT JOIN roles r ON u.role_id = r.id
             WHERE u.username = ?1 AND u.is_active = 1",
        )
        .map_err(|e| e.to_string())?;

    let user_row = stmt.query_row([username], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, Option<i64>>(4)?,
            row.get::<_, Option<String>>(5)?,
            row.get::<_, f64>(6)?,
            row.get::<_, bool>(7)?,
        ))
    });

    match user_row {
        Ok((id, uname, dname, hash, role_id, role_name, max_disc, active)) => {
            let parsed_hash = PasswordHash::new(&hash).map_err(|e| e.to_string())?;
            if Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok() {
                // Fetch permissions
                let mut perm_stmt = conn
                    .prepare(
                        "SELECT p.code FROM permissions p
                         INNER JOIN role_permissions rp ON p.id = rp.permission_id
                         WHERE rp.role_id = ?1",
                    )
                    .map_err(|e| e.to_string())?;

                let permissions = if let Some(rid) = role_id {
                    let rows = perm_stmt
                        .query_map([rid], |row| row.get::<_, String>(0))
                        .map_err(|e| e.to_string())?;
                    rows.filter_map(|r| r.ok()).collect()
                } else {
                    Vec::new()
                };

                // Update last_login
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
            } else {
                Ok(None)
            }
        }
        Err(_) => Ok(None),
    }
}