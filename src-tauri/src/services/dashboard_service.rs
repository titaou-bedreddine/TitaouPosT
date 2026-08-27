use crate::database::DbState;
use crate::models::DashboardStats;
use rusqlite::Result;

pub fn get_stats(db: &DbState) -> Result<DashboardStats, String> {
    let conn = db.conn.lock().unwrap();

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    let today_sales: i64 = conn
        .query_row(
            "SELECT COALESCE(SUM(total_amount), 0) FROM sales WHERE DATE(created_at) = ?1 AND status = 'completed'",
            [&today],
            |r| r.get(0),
        )
        .unwrap_or(0);

    let today_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM sales WHERE DATE(created_at) = ?1 AND status = 'completed'",
            [&today],
            |r| r.get(0),
        )
        .unwrap_or(0);

    let low_stock_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM products WHERE current_stock <= min_stock AND current_stock > 0 AND is_active = 1",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);

    let out_of_stock_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM products WHERE current_stock <= 0 AND is_active = 1",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);

    let today_expenses: i64 = conn
        .query_row(
            "SELECT COALESCE(SUM(amount), 0) FROM expenses WHERE date = ?1",
            [&today],
            |r| r.get(0),
        )
        .unwrap_or(0);

    let active_cash_expected: i64 = conn
        .query_row(
            "SELECT COALESCE(SUM(expected_cash), 0) FROM cash_sessions WHERE status = 'open'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);

    Ok(DashboardStats {
        today_sales,
        today_transactions_count: today_count,
        today_profit: today_sales - today_expenses, // Net indicator
        low_stock_count,
        out_of_stock_count,
        active_cash_expected,
        today_expenses,
    })
}
