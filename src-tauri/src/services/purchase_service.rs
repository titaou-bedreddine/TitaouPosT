use crate::database::DbState;
use crate::models::{CreatePurchaseInput, Purchase};
use rusqlite::Result;

pub fn create_purchase(db: &DbState, input: CreatePurchaseInput) -> Result<String, String> {
    let mut conn = db.conn.lock().unwrap();
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    tx.execute(
        "INSERT INTO purchases (invoice_number, supplier_id, user_id, date, subtotal, discount, tax, total, paid_amount, payment_method, status, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 'received', ?11)",
        rusqlite::params![
            input.invoice_number, input.supplier_id, input.user_id, input.date,
            input.subtotal, input.discount, input.tax, input.total, input.paid_amount,
            input.payment_method, input.notes
        ],
    )
    .map_err(|e| e.to_string())?;

    let purchase_id = tx.last_insert_rowid();

    // Process purchase items & increment stock
    for item in &input.items {
        tx.execute(
            "INSERT INTO purchase_items (purchase_id, product_id, quantity, unit_cost, discount, tax, total)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![
                purchase_id, item.product_id, item.quantity, item.unit_cost,
                item.discount, item.tax, item.total
            ],
        )
        .map_err(|e| e.to_string())?;

        // Increase product stock & update purchase cost
        tx.execute(
            "UPDATE products SET current_stock = current_stock + ?1, purchase_price = ?2 WHERE id = ?3",
            rusqlite::params![item.quantity, item.unit_cost, item.product_id],
        )
        .map_err(|e| e.to_string())?;

        tx.execute(
            "INSERT INTO inventory_movements (product_id, quantity, type, reference_type, reference_id, user_id, cost_at_time)
             VALUES (?1, ?2, 'purchase', 'purchase', ?3, ?4, ?5)",
            rusqlite::params![item.product_id, item.quantity, purchase_id, input.user_id, item.unit_cost],
        )
        .map_err(|e| e.to_string())?;
    }

    // Update supplier balance if remaining unpaid
    let remaining = input.total - input.paid_amount;
    if remaining > 0 {
        tx.execute(
            "UPDATE suppliers SET balance = balance + ?1 WHERE id = ?2",
            rusqlite::params![remaining, input.supplier_id],
        )
        .map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(input.invoice_number)
}

pub fn list_purchases(db: &DbState) -> Result<Vec<Purchase>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT p.id, p.invoice_number, p.supplier_id, s.name, p.user_id, u.display_name,
                    p.date, p.subtotal, p.discount, p.tax, p.total, p.paid_amount, p.payment_method, p.status, p.notes, p.created_at
             FROM purchases p
             LEFT JOIN suppliers s ON p.supplier_id = s.id
             LEFT JOIN users u ON p.user_id = u.id
             ORDER BY p.id DESC LIMIT 100",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Purchase {
                id: row.get(0)?,
                invoice_number: row.get(1)?,
                supplier_id: row.get(2)?,
                supplier_name: row.get(3)?,
                user_id: row.get(4)?,
                user_name: row.get(5)?,
                date: row.get(6)?,
                subtotal: row.get(7)?,
                discount: row.get(8)?,
                tax: row.get(9)?,
                total: row.get(10)?,
                paid_amount: row.get(11)?,
                payment_method: row.get(12)?,
                status: row.get(13)?,
                notes: row.get(14)?,
                created_at: row.get(15)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let list: Vec<Purchase> = rows.filter_map(|r| r.ok()).collect();
    Ok(list)
}