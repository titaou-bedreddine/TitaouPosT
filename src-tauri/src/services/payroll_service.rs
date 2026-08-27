use crate::database::DbState;
use crate::models::Payroll;
use rusqlite::Result;

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