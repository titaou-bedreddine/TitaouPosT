use crate::database::DbState;
use crate::models::{CreatePurchaseInput, Purchase, PurchaseItem};
use rusqlite::Result;

pub fn create_purchase(db: &DbState, input: CreatePurchaseInput) -> Result<String, String> {
    let mut conn = db.conn.lock().unwrap();
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    tx.execute(
        "INSERT INTO purchases (invoice_number, supplier_id, user_id, date, subtotal, discount, tax, total, paid_amount, payment_method, status, notes, terminal_name)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 'received', ?11, ?12)",
        rusqlite::params![
            input.invoice_number, input.supplier_id, input.user_id, input.date,
            input.subtotal, input.discount, input.tax, input.total, input.paid_amount,
            input.payment_method, input.notes,
            crate::network::current_stamp_terminal()
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
                    p.date, p.subtotal, p.discount, p.tax, p.total, p.paid_amount, p.payment_method, p.status, p.notes, p.created_at,
                    COALESCE(p.terminal_name, '')
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
                terminal_name: {
                    let t: String = row.get(16)?;
                    if t.is_empty() { None } else { Some(t) }
                },
            })
        })
        .map_err(|e| e.to_string())?;

    let list: Vec<Purchase> = rows.filter_map(|r| r.ok()).collect();
    Ok(list)
}
/// Purchase items for the view/details modal.
pub fn get_purchase_items(db: &DbState, purchase_id: i64) -> Result<Vec<PurchaseItem>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT pi.id, pi.purchase_id, pi.product_id, p.name_fr, p.name_ar,
                    pi.quantity, pi.unit_cost, pi.discount, pi.tax, pi.total
             FROM purchase_items pi
             LEFT JOIN products p ON pi.product_id = p.id
             WHERE pi.purchase_id = ?1 ORDER BY pi.id ASC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([purchase_id], |row| {
            Ok(PurchaseItem {
                id: row.get(0)?,
                purchase_id: row.get(1)?,
                product_id: row.get(2)?,
                product_name: row.get(3)?,
                product_name_ar: row.get(4)?,
                quantity: row.get(5)?,
                unit_cost: row.get(6)?,
                discount: row.get(7)?,
                tax: row.get(8)?,
                total: row.get(9)?,
            })
        })
        .map_err(|e| e.to_string())?;

    Ok(rows.filter_map(|r| r.ok()).collect())
}

/// Delete a purchase invoice: returns the stock it added, reverses the
/// supplier balance for the unpaid part, and removes its movements.
/// Edit an invoice IN PLACE (same principle as expense/sale editing): the
/// same row is rewritten — never a duplicate — the ORIGINAL date is kept
/// (an edit lands on the invoice's day, never today), a MODIFIED tag is
/// stamped into notes, and all side effects are reversed and rebooked:
/// stock quantities, inventory movements, purchase prices, supplier
/// balance. Fires a Telegram "purchase edited" alert.
pub fn update_purchase(
    db: &DbState,
    purchase_id: i64,
    input: CreatePurchaseInput,
    user_id: Option<i64>,
) -> Result<(), String> {
    let mut conn = db.conn.lock().unwrap();
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    // 1. REVERSE the old invoice's effects.
    let old_items: Vec<(i64, f64)> = {
        let mut stmt = tx
            .prepare("SELECT product_id, quantity FROM purchase_items WHERE purchase_id = ?1")
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([purchase_id], |r| Ok((r.get(0)?, r.get(1)?)))
            .map_err(|e| e.to_string())?;
        rows.filter_map(|r| r.ok()).collect()
    };
    for (product_id, qty) in &old_items {
        tx.execute(
            "UPDATE products SET current_stock = current_stock - ?1 WHERE id = ?2",
            rusqlite::params![qty, product_id],
        )
        .map_err(|e| e.to_string())?;
    }
    tx.execute(
        "DELETE FROM inventory_movements WHERE reference_type = 'purchase' AND reference_id = ?1",
        [purchase_id],
    )
    .map_err(|e| e.to_string())?;

    let (old_total, old_paid, old_supplier): (i64, i64, i64) = tx
        .query_row(
            "SELECT total, paid_amount, supplier_id FROM purchases WHERE id = ?1",
            [purchase_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .map_err(|e| e.to_string())?;
    let old_remaining = (old_total - old_paid).max(0);
    if old_remaining > 0 {
        tx.execute(
            "UPDATE suppliers SET balance = balance - ?1 WHERE id = ?2",
            rusqlite::params![old_remaining, old_supplier],
        )
        .map_err(|e| e.to_string())?;
    }

    // 2. REWRITE the same row (same id + invoice_number, MODIFIED tag).
    tx.execute(
        "UPDATE purchases SET
                supplier_id = ?2, user_id = ?3, date = ?4, subtotal = ?5, discount = ?6,
                tax = ?7, total = ?8, paid_amount = ?9, payment_method = ?10,
                notes = COALESCE(COALESCE(?11, notes), '') || ' | MODIFIED ' || datetime('now','localtime')
         WHERE id = ?1",
        rusqlite::params![
            purchase_id, input.supplier_id, input.user_id, input.date,
            input.subtotal, input.discount, input.tax, input.total,
            input.paid_amount, input.payment_method, input.notes,
        ],
    )
    .map_err(|e| e.to_string())?;

    // 3. REBOOK the new items (stock, purchase price, movements).
    tx.execute(
        "DELETE FROM purchase_items WHERE purchase_id = ?1",
        [purchase_id],
    )
    .map_err(|e| e.to_string())?;
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

    // 4. Supplier balance for the NEW unpaid remainder.
    let new_remaining = input.total - input.paid_amount;
    if new_remaining > 0 {
        tx.execute(
            "UPDATE suppliers SET balance = balance + ?1 WHERE id = ?2",
            rusqlite::params![new_remaining, input.supplier_id],
        )
        .map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;
    drop(conn);

    {
        let lang = crate::services::notifier_service::ui_language(db);
        let actor = crate::services::notifier_service::actor_label(db, user_id);
        let text = crate::services::notifier_service::tr(
            &lang,
            (
                format!("✏️ *Purchase Edited* — Invoice #{} modified in place (date kept), stock & supplier balance re-adjusted
💰 New Total: {} DZD (paid {})
👤 By: {}", input.invoice_number, input.total, input.paid_amount, actor),
                format!("✏️ *تعديل فاتورة شراء* — الفاتورة #{} عُدّلت في مكانها (نفس التاريخ)، تم تصحيح المخزون ورصيد المورد
💰 المجموع الجديد: {} دج (مدفوع {})
👤 بواسطة: {}", input.invoice_number, input.total, input.paid_amount, actor),
                format!("✏️ *Achat Modifié* — Facture #{} modifiée sur place (date conservée), stock et balance fournisseur ajustés
💰 Nouveau total : {} DZD (payé {})
👤 Par : {}", input.invoice_number, input.total, input.paid_amount, actor),
            ),
        );
        crate::services::notifier_service::notify_if_enabled(db, "notify_history_change", text);
    }

    Ok(())
}

pub fn delete_purchase(db: &DbState, purchase_id: i64, user_id: Option<i64>) -> Result<(), String> {
    let mut conn = db.conn.lock().unwrap();
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    // Reverse stock & remove inventory movements line by line.
    let items: Vec<(i64, f64)> = {
        let mut stmt = tx
            .prepare("SELECT product_id, quantity FROM purchase_items WHERE purchase_id = ?1")
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([purchase_id], |r| Ok((r.get(0)?, r.get(1)?)))
            .map_err(|e| e.to_string())?;
        rows.filter_map(|r| r.ok()).collect()
    };

    for (product_id, qty) in items {
        tx.execute(
            "UPDATE products SET current_stock = current_stock - ?1 WHERE id = ?2",
            rusqlite::params![qty, product_id],
        )
        .map_err(|e| e.to_string())?;
    }

    tx.execute(
        "DELETE FROM inventory_movements WHERE reference_type = 'purchase' AND reference_id = ?1",
        [purchase_id],
    )
    .map_err(|e| e.to_string())?;

    // Reverse the unpaid part still on the supplier's balance.
    let (total, paid, supplier_id): (i64, i64, i64) = tx
        .query_row(
            "SELECT total, paid_amount, supplier_id FROM purchases WHERE id = ?1",
            [purchase_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .map_err(|e| e.to_string())?;
    let remaining = (total - paid).max(0);
    if remaining > 0 {
        tx.execute(
            "UPDATE suppliers SET balance = balance - ?1 WHERE id = ?2",
            rusqlite::params![remaining, supplier_id],
        )
        .map_err(|e| e.to_string())?;
    }

    tx.execute("DELETE FROM purchases WHERE id = ?1", [purchase_id])
        .map_err(|e| e.to_string())?;

    tx.commit().map_err(|e| e.to_string())?;
    drop(conn);

    {
        let lang = crate::services::notifier_service::ui_language(db);
        let actor = crate::services::notifier_service::actor_label(db, user_id);
        let text = crate::services::notifier_service::tr(
            &lang,
            (
                format!("🗑 *Purchase Deleted* — Purchase #{} cancelled, stock & supplier balance reverted
👤 By: {}", purchase_id, actor),
                format!("🗑 *حذف عملية شراء* — الفاتورة #{} أُلغيت، تم تصحيح المخزون ورصيد المورد
👤 بواسطة: {}", purchase_id, actor),
                format!("🗑 *Achat Supprimé* — Achat #{} annulé, stock et balance fournisseur corrigés
👤 Par : {}", purchase_id, actor),
            ),
        );
        crate::services::notifier_service::notify_if_enabled(db, "notify_history_change", text);
    }

    Ok(())
}

#[cfg(test)]
mod edit_tests {
    use super::*;
    use crate::models::PurchaseItemInput;

    fn fresh_db() -> DbState {
        let dir = std::env::temp_dir().join("titaou_purchase_tests");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join(format!(
            "pur_{}_{}.sqlite",
            std::process::id(),
            std::thread::current().name().unwrap_or("t").replace("::", "_")
        ));
        let _ = std::fs::remove_file(&path);
        let state = DbState { conn: std::sync::Mutex::new(rusqlite::Connection::open(&path).unwrap()) };
        state.run_migrations().unwrap();
        state.seed_default_admin().unwrap();
        {
            let conn = state.conn.lock().unwrap();
            let _ = conn.execute("INSERT OR IGNORE INTO suppliers (id, name, balance, is_active) VALUES (1, 'Test Supplier', 0, 1)", []);
            conn.execute(
                "INSERT INTO products (id, name_ar, name_fr, name_en, purchase_price, sale_price, current_stock, min_stock, is_active, is_bundle, tax_rate, min_sale_price) VALUES (1, 'منتج تجريبي', 'Test Product', 'Test Product', 100, 150, 0, 5, 1, 0, 0, 90)",
                [],
            ).expect("seed product");
        }
        state
    }

    fn input(qty: f64, unit_cost: i64, total: i64, paid: i64, date: &str) -> CreatePurchaseInput {
        CreatePurchaseInput {
            invoice_number: "ACH-1".into(),
            supplier_id: 1,
            user_id: 1,
            date: date.into(),
            subtotal: total,
            discount: 0,
            tax: 0,
            total,
            paid_amount: paid,
            payment_method: "cash".into(),
            items: vec![PurchaseItemInput {
                product_id: 1,
                quantity: qty,
                unit_cost,
                discount: 0,
                tax: 0,
                total,
                expiry_date: None,
                batch_number: None,
            }],
            notes: None,
        }
    }

    #[test]
    fn purchase_edit_updates_in_place_keeps_date_rebooks_stock() {
        let db = fresh_db();
        let yesterday = (chrono::Local::now() - chrono::Duration::days(1))
            .format("%Y-%m-%d").to_string();
        create_purchase(&db, input(10.0, 100, 1000, 400, &yesterday)).unwrap();
        let pid: i64 = {
            let conn = db.conn.lock().unwrap();
            conn.query_row("SELECT id FROM purchases WHERE invoice_number='ACH-1'", [], |r| r.get(0)).unwrap()
        };

        // EDIT: 10 pcs @100 (total 1000) → 12 pcs @100 (total 1200), same date.
        update_purchase(&db, pid, input(12.0, 100, 1200, 400, &yesterday), Some(1)).unwrap();

        let conn = db.conn.lock().unwrap();
        let (count, total, date, notes): (i64, i64, String, String) = conn
            .query_row(
                "SELECT COUNT(*), MIN(total), MIN(date), MIN(notes) FROM purchases WHERE invoice_number='ACH-1'",
                [], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
            ).unwrap();
        assert_eq!(count, 1, "edit must not duplicate the invoice");
        assert_eq!(total, 1200);
        assert_eq!(date, yesterday, "date must stay on the invoice's original day");
        assert!(notes.contains("MODIFIED"), "MODIFIED tag missing");
        // Stock: old 10 reversed, new 12 booked.
        let stock: f64 = conn.query_row("SELECT current_stock FROM products WHERE id=1", [], |r| r.get(0)).unwrap();
        assert_eq!(stock, 12.0);
        // Supplier balance: old unpaid 600 reversed, new unpaid 800 applied.
        let bal: i64 = conn.query_row("SELECT balance FROM suppliers WHERE id=1", [], |r| r.get(0)).unwrap();
        assert_eq!(bal, 800);
        // Exactly one inventory movement for this purchase.
        let movs: i64 = conn.query_row(
            "SELECT COUNT(*) FROM inventory_movements WHERE reference_type='purchase' AND reference_id=?1",
            [pid], |r| r.get(0)).unwrap();
        assert_eq!(movs, 1);
    }

    #[test]
    fn purchase_delete_reverts_everything() {
        let db = fresh_db();
        create_purchase(&db, input(5.0, 100, 500, 200, "2026-09-09")).unwrap();
        let pid: i64 = {
            let conn = db.conn.lock().unwrap();
            conn.query_row("SELECT id FROM purchases WHERE invoice_number='ACH-1'", [], |r| r.get(0)).unwrap()
        };
        delete_purchase(&db, pid, Some(1)).unwrap();
        let conn = db.conn.lock().unwrap();
        let (pcount, stock, bal, movs): (i64, f64, i64, i64) = conn.query_row(
            "SELECT (SELECT COUNT(*) FROM purchases), (SELECT current_stock FROM products WHERE id=1), (SELECT balance FROM suppliers WHERE id=1), (SELECT COUNT(*) FROM inventory_movements)",
            [], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
        ).unwrap();
        assert_eq!(pcount, 0);
        assert_eq!(stock, 0.0, "stock fully reverted");
        assert_eq!(bal, 0, "supplier balance reverted");
        assert_eq!(movs, 0);
    }
}
