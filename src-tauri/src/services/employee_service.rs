use crate::database::DbState;
use crate::models::{Employee, User};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use rusqlite::Result;

pub fn list_employees(db: &DbState) -> Result<Vec<Employee>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT id, employee_code, full_name, phone, email, national_id, job_title, base_salary, salary_type, salary_start_date, hire_date, qr_code, is_active, notes
             FROM employees
             WHERE is_active = 1
             ORDER BY id DESC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Employee {
                id: row.get(0)?,
                employee_code: row.get(1)?,
                full_name: row.get(2)?,
                phone: row.get(3)?,
                email: row.get(4)?,
                national_id: row.get(5)?,
                job_title: row.get(6)?,
                base_salary: row.get(7)?,
                salary_type: row.get(8)?,
                salary_start_date: row.get(9)?,
                hire_date: row.get(10)?,
                qr_code: row.get(11)?,
                is_active: row.get(12)?,
                notes: row.get(13)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let list: Vec<Employee> = rows.filter_map(|r| r.ok()).collect();
    Ok(list)
}

pub fn save_employee(
    db: &DbState,
    code: &str,
    name: &str,
    phone: Option<String>,
    email: Option<String>,
    national_id: Option<String>,
    job_title: &str,
    base_salary: i64,
    salary_type: &str,
    salary_start_date: Option<String>,
    hire_date: &str,
    notes: Option<String>,
    employee_id: Option<i64>,
) -> Result<i64, String> {
    let conn = db.conn.lock().unwrap();

    if let Some(eid) = employee_id {
        conn.execute(
            "UPDATE employees
             SET employee_code = ?1, full_name = ?2, phone = ?3, email = ?4, national_id = ?5,
                 job_title = ?6, base_salary = ?7, salary_type = ?8, salary_start_date = ?9, hire_date = ?10, notes = ?11
             WHERE id = ?12",
            rusqlite::params![
                code, name, phone, email, national_id, job_title,
                base_salary, salary_type, salary_start_date, hire_date, notes, eid
            ],
        )
        .map_err(|e| e.to_string())?;
        Ok(eid)
    } else {
        let qr_code = format!("EMP-QR-{}", code);
        conn.execute(
            "INSERT INTO employees (employee_code, full_name, phone, email, national_id, job_title, base_salary, salary_type, salary_start_date, hire_date, qr_code, notes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            rusqlite::params![
                code, name, phone, email, national_id, job_title,
                base_salary, salary_type, salary_start_date, hire_date, qr_code, notes
            ],
        )
        .map_err(|e| e.to_string())?;
        Ok(conn.last_insert_rowid())
    }
}

pub fn delete_employee(db: &DbState, employee_id: i64) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();
    conn.execute("UPDATE employees SET is_active = 0 WHERE id = ?1", [employee_id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn change_user_password(db: &DbState, user_id: i64, new_password: &str) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(new_password.as_bytes(), &salt)
        .map_err(|e| e.to_string())?
        .to_string();

    conn.execute(
        "UPDATE users SET password_hash = ?1 WHERE id = ?2",
        rusqlite::params![password_hash, user_id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn get_user_by_qr(db: &DbState, qr_code: &str) -> Result<Option<User>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT u.id, u.username, u.display_name, u.role_id, r.name, u.max_discount_percent, u.is_active
             FROM employees e
             JOIN users u ON e.user_account_id = u.id OR u.username = LOWER(REPLACE(e.employee_code, '-', ''))
             LEFT JOIN roles r ON u.role_id = r.id
             WHERE e.qr_code = ?1 AND e.is_active = 1 AND u.is_active = 1",
        )
        .map_err(|e| e.to_string())?;

    let row = stmt.query_row([qr_code], |r| {
        Ok(User {
            id: r.get(0)?,
            username: r.get(1)?,
            display_name: r.get(2)?,
            role_id: r.get(3)?,
            role_name: r.get(4)?,
            max_discount_percent: r.get(5)?,
            is_active: r.get(6)?,
            permissions: vec!["sales.create".to_string(), "sales.view".to_string()],
        })
    });

    match row {
        Ok(u) => Ok(Some(u)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}