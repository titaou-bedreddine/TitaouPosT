use crate::database::DbState;
use crate::models::{Employee, Payroll};
use rusqlite::Result;

pub fn list_employees(db: &DbState) -> Result<Vec<Employee>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT id, employee_code, full_name, phone, email, national_id, job_title, base_salary, salary_type, hire_date, is_active, notes
             FROM employees
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
                hire_date: row.get(9)?,
                is_active: row.get(10)?,
                notes: row.get(11)?,
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
    hire_date: &str,
    notes: Option<String>,
    employee_id: Option<i64>,
) -> Result<i64, String> {
    let conn = db.conn.lock().unwrap();

    if let Some(eid) = employee_id {
        conn.execute(
            "UPDATE employees
             SET employee_code = ?1, full_name = ?2, phone = ?3, email = ?4, national_id = ?5,
                 job_title = ?6, base_salary = ?7, salary_type = ?8, hire_date = ?9, notes = ?10
             WHERE id = ?11",
            rusqlite::params![
                code, name, phone, email, national_id, job_title,
                base_salary, salary_type, hire_date, notes, eid
            ],
        )
        .map_err(|e| e.to_string())?;
        Ok(eid)
    } else {
        conn.execute(
            "INSERT INTO employees (employee_code, full_name, phone, email, national_id, job_title, base_salary, salary_type, hire_date, notes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![
                code, name, phone, email, national_id, job_title,
                base_salary, salary_type, hire_date, notes
            ],
        )
        .map_err(|e| e.to_string())?;
        Ok(conn.last_insert_rowid())
    }
}

pub fn list_payrolls(db: &DbState) -> Result<Vec<Payroll>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT p.id, p.payroll_number, p.employee_id, e.full_name, p.period_month, p.period_year,
                    p.base_salary, p.bonuses, p.allowances, p.deductions, p.advances_deducted,
                    p.net_salary, p.payment_status, p.payment_method, p.paid_at, p.notes
             FROM payrolls p
             LEFT JOIN employees e ON p.employee_id = e.id
             ORDER BY p.id DESC LIMIT 200",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Payroll {
                id: row.get(0)?,
                payroll_number: row.get(1)?,
                employee_id: row.get(2)?,
                employee_name: row.get(3)?,
                period_month: row.get(4)?,
                period_year: row.get(5)?,
                base_salary: row.get(6)?,
                bonuses: row.get(7)?,
                allowances: row.get(8)?,
                deductions: row.get(9)?,
                advances_deducted: row.get(10)?,
                net_salary: row.get(11)?,
                payment_status: row.get(12)?,
                payment_method: row.get(13)?,
                paid_at: row.get(14)?,
                notes: row.get(15)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let list: Vec<Payroll> = rows.filter_map(|r| r.ok()).collect();
    Ok(list)
}
