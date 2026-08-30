use crate::database::DbState;
use crate::models::{Category, Product, ProductInput, Unit};
use rusqlite::Result;

pub fn search_products(
    db: &DbState,
    query: &str,
    category_id: Option<i64>,
    search_type: &str,
) -> Result<Vec<Product>, String> {
    let conn = db.conn.lock().unwrap();
    let q = query.trim();

    let mut sql = String::from(
        "SELECT DISTINCT p.id, p.sku, p.name_ar, p.name_fr, p.name_en, p.category_id, c.name_ar,
                p.unit_id, u.name, p.purchase_price, p.sale_price, p.min_sale_price, p.tax_rate,
                p.current_stock, p.min_stock, p.max_stock, p.image_path, p.expiry_date,
                COALESCE(p.is_scalable, 0), p.scale_code, p.scale_plu, COALESCE(p.scale_barcode_type, 97),
                COALESCE(p.scale_department_id, 1), COALESCE(p.scale_sync_status, 'pending'),
                p.is_bundle, p.is_active
         FROM products p
         LEFT JOIN categories c ON p.category_id = c.id
         LEFT JOIN units u ON p.unit_id = u.id
         LEFT JOIN product_barcodes pb ON p.id = pb.product_id
         WHERE p.is_active = 1"
    );

    if let Some(cid) = category_id {
        sql.push_str(&format!(" AND p.category_id = {}", cid));
    }

    if !q.is_empty() {
        match search_type {
            "name" => {
                sql.push_str(&format!(" AND (p.name_ar LIKE '%{0}%' OR p.name_fr LIKE '%{0}%' OR p.name_en LIKE '%{0}%')", q));
            }
            "barcode" => {
                sql.push_str(&format!(" AND (p.sku LIKE '%{0}%' OR pb.barcode LIKE '%{0}%')", q));
            }
            "price" => {
                if let Ok(price_val) = q.parse::<i64>() {
                    sql.push_str(&format!(" AND p.sale_price = {}", price_val));
                }
            }
            _ => {
                sql.push_str(&format!(
                    " AND (p.name_ar LIKE '%{0}%' OR p.name_fr LIKE '%{0}%' OR p.name_en LIKE '%{0}%' OR p.sku LIKE '%{0}%' OR pb.barcode LIKE '%{0}%')",
                    q
                ));
            }
        }
    }

    sql.push_str(" ORDER BY p.id ASC LIMIT 1000");

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

    let product_rows = stmt
        .query_map([], |row| {
            let is_scalable_int: i64 = row.get(18)?;
            Ok(Product {
                id: row.get(0)?,
                sku: row.get(1)?,
                name_ar: row.get(2)?,
                name_fr: row.get(3)?,
                name_en: row.get(4)?,
                category_id: row.get(5)?,
                category_name: row.get(6)?,
                unit_id: row.get(7)?,
                unit_name: row.get(8)?,
                purchase_price: row.get(9)?,
                sale_price: row.get(10)?,
                min_sale_price: row.get(11)?,
                tax_rate: row.get(12)?,
                current_stock: row.get(13)?,
                min_stock: row.get(14)?,
                max_stock: row.get(15)?,
                image_path: row.get(16)?,
                expiry_date: row.get(17)?,
                is_scalable: is_scalable_int == 1,
                scale_code: row.get(19)?,
                scale_plu: row.get(20)?,
                scale_barcode_type: row.get(21)?,
                scale_department_id: row.get(22)?,
                scale_sync_status: row.get(23)?,
                is_bundle: row.get(24)?,
                is_active: row.get(25)?,
                barcodes: Vec::new(),
            })
        })
        .map_err(|e| e.to_string())?;

    let mut products = Vec::new();
    for p_res in product_rows {
        if let Ok(mut p) = p_res {
            let mut b_stmt = conn
                .prepare("SELECT barcode FROM product_barcodes WHERE product_id = ?1 ORDER BY is_primary DESC")
                .map_err(|e| e.to_string())?;
            let barcodes = b_stmt
                .query_map([p.id], |r| r.get(0))
                .map_err(|e| e.to_string())?
                .filter_map(|r| r.ok())
                .collect();
            p.barcodes = barcodes;
            products.push(p);
        }
    }

    Ok(products)
}

pub fn save_product(db: &DbState, input: ProductInput, product_id: Option<i64>) -> Result<i64, String> {
    if input.purchase_price <= 0 {
        return Err("Purchase price must be greater than 0 / سعر الشراء إجباري وأكبر من الصفر".to_string());
    }
    if input.sale_price <= 0 {
        return Err("Sale price must be greater than 0 / سعر البيع إجباري وأكبر من الصفر".to_string());
    }
    if input.sale_price < input.purchase_price {
        return Err("Sale price cannot be less than purchase price / لا يمكن أن يكون سعر البيع أقل من سعر الشراء".to_string());
    }

    let mut conn = db.conn.lock().unwrap();

    // Reject duplicate barcodes and duplicate product names across the
    // catalog. When editing, the product's own rows are excluded.
    let clean_barcodes: Vec<String> = input
        .barcodes
        .iter()
        .map(|b| b.trim().to_string())
        .filter(|b| !b.is_empty())
        .collect();

    for b in &clean_barcodes {
        let owner: Option<i64> = conn
            .query_row(
                "SELECT product_id FROM product_barcodes WHERE barcode = ?1
                 UNION SELECT id FROM products WHERE sku = ?1",
                [b],
                |r| r.get(0),
            )
            .map_err(|e| e.to_string())?;
        if let Some(owner_id) = owner {
            if Some(owner_id) != product_id {
                return Err(format!(
                    "Barcode {} is already used by another product / الباركود {} مستعمل من قبل منتج آخر",
                    b, b
                ));
            }
        }
    }

    for (field, name) in [
        ("name_ar", input.name_ar.trim()),
        ("name_fr", input.name_fr.trim()),
        ("name_en", input.name_en.trim()),
    ] {
        if name.is_empty() {
            continue;
        }
        let owner: Option<i64> = conn
            .query_row(
                &format!("SELECT id FROM products WHERE {} = ?1 AND id != ?2", field),
                rusqlite::params![name, product_id.unwrap_or(-1)],
                |r| r.get(0),
            )
            .map_err(|e| e.to_string())?;
        if owner.is_some() {
            return Err(format!(
                "Product name \"{}\" already exists / اسم المنتج \"{}\" موجود مسبقاً",
                name, name
            ));
        }
    }

    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let is_scalable_int = if input.is_scalable { 1 } else { 0 };

    let id = if let Some(pid) = product_id {
        // Fetch old prices for price history
        let old_prices: Option<(i64, i64)> = tx.query_row(
            "SELECT purchase_price, sale_price FROM products WHERE id = ?1",
            [pid],
            |r| Ok((r.get(0)?, r.get(1)?)),
        ).ok();

        if let Some((old_pur, old_sale)) = old_prices {
            if old_pur != input.purchase_price || old_sale != input.sale_price {
                let _ = tx.execute(
                    "INSERT INTO product_price_history (product_id, old_purchase_price, new_purchase_price, old_sale_price, new_sale_price)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    rusqlite::params![pid, old_pur, input.purchase_price, old_sale, input.sale_price],
                );
            }
        }

        tx.execute(
            "UPDATE products SET
                sku = ?1, name_ar = ?2, name_fr = ?3, name_en = ?4,
                category_id = ?5, unit_id = ?6, purchase_price = ?7,
                sale_price = ?8, min_sale_price = ?9, tax_rate = ?10,
                current_stock = ?11, min_stock = ?12, image_path = ?13,
                expiry_date = ?14, is_scalable = ?15, scale_code = ?16,
                scale_plu = ?17, scale_barcode_type = ?18, scale_department_id = ?19,
                scale_sync_status = ?20, is_bundle = ?21
             WHERE id = ?22",
            rusqlite::params![
                input.sku,
                input.name_ar,
                input.name_fr,
                input.name_en,
                input.category_id,
                input.unit_id,
                input.purchase_price,
                input.sale_price,
                input.min_sale_price,
                input.tax_rate,
                input.current_stock,
                input.min_stock,
                input.image_path,
                input.expiry_date,
                is_scalable_int,
                input.scale_code,
                input.scale_plu,
                input.scale_barcode_type,
                input.scale_department_id,
                input.scale_sync_status.unwrap_or_else(|| "pending".to_string()),
                input.is_bundle,
                pid
            ],
        )
        .map_err(|e| e.to_string())?;

        tx.execute(
            "DELETE FROM product_barcodes WHERE product_id = ?1",
            [pid],
        )
        .map_err(|e| e.to_string())?;

        pid
    } else {
        tx.execute(
            "INSERT INTO products (
                sku, name_ar, name_fr, name_en, category_id, unit_id,
                purchase_price, sale_price, min_sale_price, tax_rate,
                current_stock, min_stock, image_path, expiry_date,
                is_scalable, scale_code, scale_plu, scale_barcode_type,
                scale_department_id, scale_sync_status, is_bundle, is_active
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, 1)",
            rusqlite::params![
                input.sku,
                input.name_ar,
                input.name_fr,
                input.name_en,
                input.category_id,
                input.unit_id,
                input.purchase_price,
                input.sale_price,
                input.min_sale_price,
                input.tax_rate,
                input.current_stock,
                input.min_stock,
                input.image_path,
                input.expiry_date,
                is_scalable_int,
                input.scale_code,
                input.scale_plu,
                input.scale_barcode_type,
                input.scale_department_id,
                input.scale_sync_status.unwrap_or_else(|| "pending".to_string()),
                input.is_bundle
            ],
        )
        .map_err(|e| e.to_string())?;

        let new_id = tx.last_insert_rowid();

        let _ = tx.execute(
            "INSERT INTO product_price_history (product_id, old_purchase_price, new_purchase_price, old_sale_price, new_sale_price)
             VALUES (?1, 0, ?2, 0, ?3)",
            rusqlite::params![new_id, input.purchase_price, input.sale_price],
        );

        new_id
    };

    for (i, b) in clean_barcodes.iter().enumerate() {
        let is_primary = i == 0;
        let _ = tx.execute(
            "INSERT OR IGNORE INTO product_barcodes (product_id, barcode, is_primary) VALUES (?1, ?2, ?3)",
            rusqlite::params![id, b, is_primary],
        );
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(id)
}

pub fn delete_product(db: &DbState, product_id: i64) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();
    conn.execute("UPDATE products SET is_active = 0 WHERE id = ?1", [product_id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_categories(db: &DbState) -> Result<Vec<Category>, String> {
    let conn = db.conn.lock().unwrap();
    let _ = conn.execute("ALTER TABLE categories ADD COLUMN color TEXT DEFAULT '#0284c7';", []);

    let mut stmt = conn
        .prepare("SELECT id, parent_id, name_ar, name_fr, name_en, COALESCE(color, '#0284c7'), is_active FROM categories WHERE is_active = 1 ORDER BY id ASC")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Category {
                id: row.get(0)?,
                parent_id: row.get(1)?,
                name_ar: row.get(2)?,
                name_fr: row.get(3)?,
                name_en: row.get(4)?,
                color: row.get(5)?,
                is_active: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let list: Vec<Category> = rows.filter_map(|r| r.ok()).collect();
    Ok(list)
}

pub fn save_category(db: &DbState, name_ar: &str, name_fr: &str, name_en: &str, color: &str, category_id: Option<i64>) -> Result<i64, String> {
    let conn = db.conn.lock().unwrap();
    let _ = conn.execute("ALTER TABLE categories ADD COLUMN color TEXT DEFAULT '#0284c7';", []);

    if let Some(cid) = category_id {
        conn.execute(
            "UPDATE categories SET name_ar = ?1, name_fr = ?2, name_en = ?3, color = ?4 WHERE id = ?5",
            rusqlite::params![name_ar, name_fr, name_en, color, cid],
        ).map_err(|e| e.to_string())?;
        Ok(cid)
    } else {
        conn.execute(
            "INSERT INTO categories (name_ar, name_fr, name_en, color, is_active) VALUES (?1, ?2, ?3, ?4, 1)",
            rusqlite::params![name_ar, name_fr, name_en, color],
        ).map_err(|e| e.to_string())?;
        Ok(conn.last_insert_rowid())
    }
}

pub fn delete_category(db: &DbState, category_id: i64) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();
    conn.execute("UPDATE categories SET is_active = 0 WHERE id = ?1", [category_id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_units(db: &DbState) -> Result<Vec<Unit>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, name, short_name, allow_decimals FROM units ORDER BY id ASC")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Unit {
                id: row.get(0)?,
                name: row.get(1)?,
                short_name: row.get(2)?,
                allow_decimals: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let list: Vec<Unit> = rows.filter_map(|r| r.ok()).collect();
    Ok(list)
}

pub fn save_unit(db: &DbState, name: &str, short_name: &str, allow_decimals: bool, unit_id: Option<i64>) -> Result<i64, String> {
    let conn = db.conn.lock().unwrap();
    if let Some(uid) = unit_id {
        conn.execute(
            "UPDATE units SET name = ?1, short_name = ?2, allow_decimals = ?3 WHERE id = ?4",
            rusqlite::params![name, short_name, allow_decimals, uid],
        )
        .map_err(|e| e.to_string())?;
        Ok(uid)
    } else {
        conn.execute(
            "INSERT INTO units (name, short_name, allow_decimals) VALUES (?1, ?2, ?3)",
            rusqlite::params![name, short_name, allow_decimals],
        )
        .map_err(|e| e.to_string())?;
        Ok(conn.last_insert_rowid())
    }
}