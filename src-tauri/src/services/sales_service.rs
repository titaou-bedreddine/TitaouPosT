use crate::database::DbState;
use crate::models::{CartItem, CreateSaleInput, HeldSale, Sale};
use rusqlite::Result;

pub fn process_sale(db: &DbState, input: CreateSaleInput) -> Result<String, String> {
    let mut conn = db.conn.lock().unwrap();
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let now = chrono::Local::now();
    let sale_number = format!("POS-{}", now.format("%Y%m%d%H%M%S"));

    let payment_status = if input.paid_amount >= input.total_amount {
        "paid"
    } else if input.paid_amount > 0 {
        "partial"
    } else {
        "credit"
    };

    tx.execute(
        "INSERT INTO sales (sale_number, session_id, user_id, customer_id, subtotal, discount_amount, discount_percentage, discount_reason, tax_amount, total_amount, paid_amount, change_amount, payment_status, status, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, 'completed', ?14)",
        rusqlite::params![
            sale_number, input.session_id, input.user_id, input.customer_id,
            input.subtotal, input.discount_amount, input.discount_percentage,
            input.discount_reason, input.tax_amount, input.total_amount,
            input.paid_amount, input.change_amount, payment_status, input.notes
        ],
    )
    .map_err(|e| e.to_string())?;

    let sale_id = tx.last_insert_rowid();

    let is_versement = input
        .payment_method
        .as_deref()
        .map(|m| m == "versement")
        .unwrap_or(false);
    // Versement/layaway: goods physically stay at the shop, stock is only
    // decremented when the sale is fully paid and released.
    let skip_stock = input.skip_stock || is_versement;

    // Process each cart item & update stock
    for item in &input.items {
        tx.execute(
            "INSERT INTO sale_items (sale_id, product_id, quantity, unit_price, discount_amount, tax_amount, total_price, is_refunded, refunded_quantity)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![
                sale_id, item.product_id, item.quantity, item.unit_price,
                item.discount_amount, item.tax_amount, item.total_price,
                item.is_refund, if item.is_refund { item.quantity } else { 0.0 }
            ],
        )
        .map_err(|e| e.to_string())?;

        if skip_stock {
            continue;
        }

        let movement_type = if item.is_refund { "sale_refund" } else { "sale" };
        let stock_change = if item.is_refund { item.quantity } else { -item.quantity };

        tx.execute(
            "UPDATE products SET current_stock = current_stock + ?1 WHERE id = ?2",
            rusqlite::params![stock_change, item.product_id],
        )
        .map_err(|e| e.to_string())?;

        tx.execute(
            "INSERT INTO inventory_movements (product_id, quantity, type, reference_type, reference_id, user_id)
             VALUES (?1, ?2, ?3, 'sale', ?4, ?5)",
            rusqlite::params![item.product_id, stock_change, movement_type, sale_id, input.user_id],
        )
        .map_err(|e| e.to_string())?;
    }

    // Process payment breakdown
    let mut total_cash_paid: i64 = 0;

    let payments_list: Vec<crate::models::SalePaymentInput> = if input.payments.is_empty() {
        let method = input.payment_method.clone().unwrap_or_else(|| "cash".to_string());
        vec![crate::models::SalePaymentInput {
            payment_method: method,
            amount: input.total_amount,
            reference_code: None,
        }]
    } else {
        input.payments.clone()
    };

    for payment in &payments_list {
        tx.execute(
            "INSERT INTO sale_payments (sale_id, payment_method, amount, reference_code)
             VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![sale_id, payment.payment_method, payment.amount, payment.reference_code],
        )
        .map_err(|e| e.to_string())?;

        if payment.payment_method == "cash" {
            total_cash_paid += payment.amount;
        }
    }

    let net_cash_change = total_cash_paid - input.change_amount;
    if net_cash_change != 0 {
        tx.execute(
            "INSERT INTO cash_movements (session_id, user_id, type, amount, reason, reference_type, reference_id)
             VALUES (?1, ?2, 'cash_sale', ?3, ?4, 'sale', ?5)",
            rusqlite::params![
                input.session_id, input.user_id, net_cash_change,
                format!("POS Sale / بيع {}", sale_number), sale_id
            ],
        )
        .map_err(|e| e.to_string())?;

        tx.execute(
            "UPDATE cash_sessions SET expected_cash = expected_cash + ?1 WHERE id = ?2",
            rusqlite::params![net_cash_change, input.session_id],
        )
        .map_err(|e| e.to_string())?;
    }

    // Credit & versement both track the unpaid remainder on the customer's
    // account. Credit: goods leave now, total owed minus paid now. Versement:
    // goods stay at the shop, same remainder math.
    let owed_remainder = (input.total_amount - input.paid_amount.clamp(0, input.total_amount)).max(0);
    if owed_remainder > 0 {
        if let Some(cust_id) = input.customer_id {
            tx.execute(
                "UPDATE customers SET balance = balance + ?1 WHERE id = ?2",
                rusqlite::params![owed_remainder, cust_id],
            )
            .map_err(|e| e.to_string())?;
        }
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(sale_number)
}

pub fn list_sales(
    db: &DbState,
    start_date: Option<String>,
    end_date: Option<String>,
    user_id: Option<i64>,
    limit: i64,
) -> Result<Vec<Sale>, String> {
    let conn = db.conn.lock().unwrap();
    let mut sql = String::from(
        "SELECT s.id, s.sale_number, s.session_id, s.user_id, u.display_name,
                s.customer_id, c.name, s.subtotal, s.discount_amount, s.tax_amount,
                s.total_amount, s.paid_amount, s.change_amount, s.payment_status, s.status, s.created_at,
                (SELECT sp.payment_method FROM sale_payments sp WHERE sp.sale_id = s.id ORDER BY sp.amount DESC LIMIT 1) as payment_method
         FROM sales s
         LEFT JOIN users u ON s.user_id = u.id
         LEFT JOIN customers c ON s.customer_id = c.id
         WHERE 1=1"
    );

    if let Some(ref sd) = start_date {
        if !sd.is_empty() {
            sql.push_str(&format!(" AND DATE(s.created_at) >= '{}'", sd));
        }
    }

    if let Some(ref ed) = end_date {
        if !ed.is_empty() {
            sql.push_str(&format!(" AND DATE(s.created_at) <= '{}'", ed));
        }
    }

    if let Some(uid) = user_id {
        sql.push_str(&format!(" AND s.user_id = {}", uid));
    }

    sql.push_str(&format!(" ORDER BY s.id DESC LIMIT {}", limit));

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Sale {
                id: row.get(0)?,
                sale_number: row.get(1)?,
                session_id: row.get(2)?,
                user_id: row.get(3)?,
                user_name: row.get(4)?,
                customer_id: row.get(5)?,
                customer_name: row.get(6)?,
                subtotal: row.get(7)?,
                discount_amount: row.get(8)?,
                tax_amount: row.get(9)?,
                total_amount: row.get(10)?,
                paid_amount: row.get(11)?,
                change_amount: row.get(12)?,
                payment_status: row.get(13)?,
                status: row.get(14)?,
                created_at: row.get(15)?,
                payment_method: row.get(16)?,
                items: None,
            })
        })
        .map_err(|e| e.to_string())?;

    let list: Vec<Sale> = rows.filter_map(|r| r.ok()).collect();
    Ok(list)
}

pub fn get_sale_items(db: &DbState, sale_id: i64) -> Result<Vec<CartItem>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT si.product_id, p.sku, '', p.name_ar, p.name_fr, p.name_en, p.image_path,
                    si.unit_price, si.quantity, si.discount_amount, si.tax_amount, si.total_price, si.is_refunded
             FROM sale_items si
             JOIN products p ON si.product_id = p.id
             WHERE si.sale_id = ?1",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([sale_id], |row| {
            Ok(CartItem {
                product_id: row.get(0)?,
                sku: row.get(1)?,
                barcode: None,
                name_ar: row.get(3)?,
                name_fr: row.get(4)?,
                name_en: row.get(5)?,
                image_path: row.get(6)?,
                unit_price: row.get(7)?,
                quantity: row.get(8)?,
                discount_amount: row.get(9)?,
                tax_amount: row.get(10)?,
                total_price: row.get(11)?,
                is_refund: row.get(12)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let list: Vec<CartItem> = rows.filter_map(|r| r.ok()).collect();
    Ok(list)
}

pub fn hold_sale(db: &DbState, user_id: i64, customer_id: Option<i64>, cart_json: &str, note: Option<String>) -> Result<i64, String> {
    let conn = db.conn.lock().unwrap();
    let now = chrono::Local::now();
    let sale_reference = format!("HELD-{}", now.format("%H%M%S"));

    conn.execute(
        "INSERT INTO held_sales (sale_reference, user_id, customer_id, cart_json, note)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![sale_reference, user_id, customer_id, cart_json, note],
    )
    .map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid())
}

pub fn list_held_sales(db: &DbState) -> Result<Vec<HeldSale>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, sale_reference, user_id, customer_id, cart_json, note, created_at FROM held_sales ORDER BY id DESC")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(HeldSale {
                id: row.get(0)?,
                sale_reference: row.get(1)?,
                user_id: row.get(2)?,
                customer_id: row.get(3)?,
                cart_json: row.get(4)?,
                note: row.get(5)?,
                created_at: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let list: Vec<HeldSale> = rows.filter_map(|r| r.ok()).collect();
    Ok(list)
}

pub fn delete_held_sale(db: &DbState, held_id: i64) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();
    conn.execute("DELETE FROM held_sales WHERE id = ?1", [held_id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn delete_sale(db: &DbState, sale_id: i64) -> Result<(), String> {
    let mut conn = db.conn.lock().unwrap();
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    // Restore stock for each item
    {
        let mut stmt = tx
            .prepare("SELECT product_id, quantity, is_refunded FROM sale_items WHERE sale_id = ?1")
            .map_err(|e| e.to_string())?;
        let items: Vec<(i64, f64, bool)> = stmt
            .query_map([sale_id], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            })
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

        for (product_id, qty, is_refund) in items {
            let stock_reversal = if is_refund { -qty } else { qty };
            tx.execute(
                "UPDATE products SET current_stock = current_stock + ?1 WHERE id = ?2",
                rusqlite::params![stock_reversal, product_id],
            )
            .map_err(|e| e.to_string())?;
        }
    }

    // Delete sale (cascade deletes sale_items and sale_payments)
    tx.execute("DELETE FROM sales WHERE id = ?1", [sale_id])
        .map_err(|e| e.to_string())?;

    tx.commit().map_err(|e| e.to_string())?;
    Ok(())
}