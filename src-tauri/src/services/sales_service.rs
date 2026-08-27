use crate::database::DbState;
use crate::models::{CreateSaleInput, Sale};
use rusqlite::Result;

pub fn process_sale(db: &DbState, input: CreateSaleInput) -> Result<String, String> {
    let mut conn = db.conn.lock().unwrap();
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    // Generate unique sale number: e.g. SALE-20260827-XXXX
    let now = chrono::Local::now();
    let timestamp_str = now.format("%Y%m%d%H%M%S").to_string();
    let sale_number = format!("POS-{}", timestamp_str);

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

    // Process each cart item & update inventory
    for item in &input.items {
        tx.execute(
            "INSERT INTO sale_items (sale_id, product_id, quantity, unit_price, discount_amount, tax_amount, total_price, is_refunded)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![
                sale_id, item.product_id, item.quantity, item.unit_price,
                item.discount_amount, item.tax_amount, item.total_price, item.is_refund
            ],
        )
        .map_err(|e| e.to_string())?;

        // Check if product is bundle
        let is_bundle: bool = tx
            .query_row("SELECT is_bundle FROM products WHERE id = ?1", [item.product_id], |r| r.get(0))
            .unwrap_or(false);

        if is_bundle {
            // Deduct component products
            let mut b_stmt = tx
                .prepare("SELECT component_product_id, quantity FROM product_bundle_items WHERE bundle_product_id = ?1")
                .map_err(|e| e.to_string())?;

            let bundle_components: Vec<(i64, f64)> = b_stmt
                .query_map([item.product_id], |r| Ok((r.get(0)?, r.get(1)?)))
                .map_err(|e| e.to_string())?
                .filter_map(|r| r.ok())
                .collect();

            for (comp_id, comp_qty) in bundle_components {
                let total_comp_qty = comp_qty * item.quantity;
                let movement_type = if item.is_refund { "bundle_refund" } else { "bundle_sale" };
                let stock_change = if item.is_refund { total_comp_qty } else { -total_comp_qty };

                tx.execute(
                    "UPDATE products SET current_stock = current_stock + ?1 WHERE id = ?2",
                    rusqlite::params![stock_change, comp_id],
                )
                .map_err(|e| e.to_string())?;

                tx.execute(
                    "INSERT INTO inventory_movements (product_id, quantity, type, reference_type, reference_id, user_id)
                     VALUES (?1, ?2, ?3, 'sale', ?4, ?5)",
                    rusqlite::params![comp_id, stock_change, movement_type, sale_id, input.user_id],
                )
                .map_err(|e| e.to_string())?;
            }
        } else {
            // Standard product stock deduction
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
    }

    // Process payment breakdown
    let mut total_cash_paid: i64 = 0;
    let mut total_credit_amount: i64 = 0;

    for payment in &input.payments {
        tx.execute(
            "INSERT INTO sale_payments (sale_id, payment_method, amount, reference_code)
             VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![sale_id, payment.payment_method, payment.amount, payment.reference_code],
        )
        .map_err(|e| e.to_string())?;

        if payment.payment_method == "cash" {
            total_cash_paid += payment.amount;
        } else if payment.payment_method == "credit" {
            total_credit_amount += payment.amount;
        }
    }

    // Subtract change from cash
    let net_cash_change = total_cash_paid - input.change_amount;
    if net_cash_change != 0 {
        // Record in cash session movements
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

    // Update customer debt if credit payment
    if total_credit_amount > 0 {
        if let Some(cust_id) = input.customer_id {
            tx.execute(
                "UPDATE customers SET balance = balance + ?1 WHERE id = ?2",
                rusqlite::params![total_credit_amount, cust_id],
            )
            .map_err(|e| e.to_string())?;
        }
    }

    // Queue notification if needed
    let payload = serde_json::json!({
        "sale_number": sale_number,
        "total_amount": input.total_amount,
        "user_id": input.user_id,
        "items_count": input.items.len()
    });

    let _ = tx.execute(
        "INSERT INTO notification_queue (channel, event_type, payload_json) VALUES ('telegram', 'sale_alert', ?1)",
        [payload.to_string()],
    );

    tx.commit().map_err(|e| e.to_string())?;
    Ok(sale_number)
}

pub fn list_sales(db: &DbState, limit: i64) -> Result<Vec<Sale>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT s.id, s.sale_number, s.session_id, s.user_id, u.display_name,
                    s.customer_id, c.name, s.subtotal, s.discount_amount, s.tax_amount,
                    s.total_amount, s.paid_amount, s.change_amount, s.payment_status, s.status, s.created_at
             FROM sales s
             LEFT JOIN users u ON s.user_id = u.id
             LEFT JOIN customers c ON s.customer_id = c.id
             ORDER BY s.id DESC LIMIT ?1",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([limit], |row| {
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
            })
        })
        .map_err(|e| e.to_string())?;

    let list: Vec<Sale> = rows.filter_map(|r| r.ok()).collect();
    Ok(list)
}
