use crate::auth::authenticate_user;
use crate::database::DbState;
use crate::models::{
    CartItem, Category, Customer, CustomerPaymentInput, DashboardStats, Employee, Expense, HeldSale,
    Payroll, Product, ProductInput, Purchase, CreatePurchaseInput, Sale, Supplier, Unit, User, UserAccount, Role,
    CashMovement, CashSession, CreateSaleInput,
};
use crate::services::{
    cash_service, customer_service, dashboard_service, employee_service, expense_service,
    payroll_service, product_service, purchase_service, sales_service, settings_service,
    supplier_service, scale_service, drawer_service, user_service,
};
use std::collections::HashMap;
use tauri::State;

#[tauri::command]
pub fn login(db: State<'_, DbState>, username: String, password: String) -> Result<Option<User>, String> {
    authenticate_user(&db, &username, &password)
}

#[tauri::command]
pub fn get_active_users(db: State<'_, DbState>) -> Result<Vec<User>, String> {
    crate::auth::list_active_users(&db)
}

#[tauri::command]
pub fn change_user_password(db: State<'_, DbState>, user_id: i64, new_password: String) -> Result<(), String> {
    employee_service::change_user_password(&db, user_id, &new_password)
}

#[tauri::command]
pub fn get_user_by_qr(db: State<'_, DbState>, qr_code: String) -> Result<Option<User>, String> {
    employee_service::get_user_by_qr(&db, &qr_code)
}

#[tauri::command]
pub fn get_dashboard_stats(db: State<'_, DbState>, start_date: Option<String>, end_date: Option<String>) -> Result<DashboardStats, String> {
    dashboard_service::get_stats(&db, start_date, end_date)
}

// Cash
#[tauri::command]
pub fn get_active_cash_session(db: State<'_, DbState>, _user_id: i64) -> Result<Option<CashSession>, String> {
    cash_service::get_active_session(&db, _user_id)
}

#[tauri::command]
pub fn open_cash_session(
    db: State<'_, DbState>,
    user_id: i64,
    register_id: i64,
    opening_amount: i64,
    notes: Option<String>,
) -> Result<CashSession, String> {
    cash_service::open_session(&db, user_id, register_id, opening_amount, notes)
}

#[tauri::command]
pub fn add_cash_movement(
    db: State<'_, DbState>,
    session_id: i64,
    user_id: i64,
    movement_type: String,
    amount: i64,
    reason: Option<String>,
) -> Result<(), String> {
    cash_service::add_cash_movement(&db, session_id, user_id, &movement_type, amount, reason)
}

#[tauri::command]
pub fn close_cash_session(
    db: State<'_, DbState>,
    session_id: i64,
    actual_cash: i64,
    notes: Option<String>,
) -> Result<(), String> {
    cash_service::close_session(&db, session_id, actual_cash, notes)
}

#[tauri::command]
pub fn list_cash_movements(db: State<'_, DbState>, session_id: i64) -> Result<Vec<CashMovement>, String> {
    cash_service::list_movements(&db, session_id)
}

#[tauri::command]
pub fn list_session_history(db: State<'_, DbState>) -> Result<Vec<CashSession>, String> {
    cash_service::list_session_history(&db)
}

// Products & Categories
#[tauri::command]
pub fn search_products(
    db: State<'_, DbState>,
    query: String,
    category_id: Option<i64>,
    search_type: String,
) -> Result<Vec<Product>, String> {
    product_service::search_products(&db, &query, category_id, &search_type)
}

#[tauri::command]
pub fn save_product(
    db: State<'_, DbState>,
    input: ProductInput,
    product_id: Option<i64>,
) -> Result<i64, String> {
    product_service::save_product(&db, input, product_id)
}

#[tauri::command]
pub fn delete_product(db: State<'_, DbState>, product_id: i64) -> Result<(), String> {
    product_service::delete_product(&db, product_id)
}

#[tauri::command]
pub fn get_categories(db: State<'_, DbState>) -> Result<Vec<Category>, String> {
    product_service::get_categories(&db)
}

#[tauri::command]
pub fn save_category(
    db: State<'_, DbState>,
    name_ar: String,
    name_fr: String,
    name_en: String,
    color: String,
    category_id: Option<i64>,
) -> Result<i64, String> {
    product_service::save_category(&db, &name_ar, &name_fr, &name_en, &color, category_id)
}

#[tauri::command]
pub fn delete_category(db: State<'_, DbState>, category_id: i64) -> Result<(), String> {
    product_service::delete_category(&db, category_id)
}

#[tauri::command]
pub fn get_units(db: State<'_, DbState>) -> Result<Vec<Unit>, String> {
    product_service::get_units(&db)
}

// Sales & Held Sales
#[tauri::command]
pub fn process_sale(db: State<'_, DbState>, input: CreateSaleInput) -> Result<String, String> {
    sales_service::process_sale(&db, input)
}

#[tauri::command]
pub fn create_sale(db: State<'_, DbState>, input: CreateSaleInput) -> Result<String, String> {
    sales_service::process_sale(&db, input)
}

#[tauri::command]
pub fn list_sales(
    db: State<'_, DbState>,
    start_date: Option<String>,
    end_date: Option<String>,
    user_id: Option<i64>,
    limit: i64,
) -> Result<Vec<Sale>, String> {
    sales_service::list_sales(&db, start_date, end_date, user_id, limit)
}

#[tauri::command]
pub fn get_sale_items(db: State<'_, DbState>, sale_id: i64) -> Result<Vec<CartItem>, String> {
    sales_service::get_sale_items(&db, sale_id)
}

#[tauri::command]
pub fn hold_sale(
    db: State<'_, DbState>,
    user_id: Option<i64>,
    customer_id: Option<i64>,
    cart_json: Option<String>,
    cart_data_json: Option<String>,
    _total_amount: Option<i64>,
    note: Option<String>,
    notes: Option<String>,
) -> Result<i64, String> {
    let uid = user_id.unwrap_or(1);
    let raw_json = cart_json.or(cart_data_json).unwrap_or_else(|| "[]".to_string());
    let final_note = note.or(notes);
    sales_service::hold_sale(&db, uid, customer_id, &raw_json, final_note)
}

#[tauri::command]
pub fn list_held_sales(db: State<'_, DbState>) -> Result<Vec<HeldSale>, String> {
    sales_service::list_held_sales(&db)
}

#[tauri::command]
pub fn delete_held_sale(db: State<'_, DbState>, held_id: i64) -> Result<(), String> {
    sales_service::delete_held_sale(&db, held_id)
}

// Customers & Debt
#[tauri::command]
pub fn list_customers(db: State<'_, DbState>) -> Result<Vec<Customer>, String> {
    customer_service::list_customers(&db)
}

#[tauri::command]
pub fn save_customer(
    db: State<'_, DbState>,
    name: String,
    phone: Option<String>,
    email: Option<String>,
    address: Option<String>,
    rc: Option<String>,
    nif: Option<String>,
    nis: Option<String>,
    ai: Option<String>,
    initial_debt: i64,
    notes: Option<String>,
    customer_id: Option<i64>,
) -> Result<i64, String> {
    customer_service::save_customer(
        &db, &name, phone, email, address, rc, nif, nis, ai, initial_debt, notes, customer_id,
    )
}

#[tauri::command]
pub fn delete_customer(db: State<'_, DbState>, customer_id: i64) -> Result<(), String> {
    customer_service::delete_customer(&db, customer_id)
}

#[tauri::command]
pub fn record_customer_debt_payment(db: State<'_, DbState>, input: CustomerPaymentInput) -> Result<i64, String> {
    customer_service::record_customer_debt_payment(&db, input)
}

// Suppliers & Purchases
#[tauri::command]
pub fn list_suppliers(db: State<'_, DbState>) -> Result<Vec<Supplier>, String> {
    supplier_service::list_suppliers(&db)
}

#[tauri::command]
pub fn save_supplier(
    db: State<'_, DbState>,
    name: String,
    contact_person: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    address: Option<String>,
    rc: Option<String>,
    nif: Option<String>,
    nis: Option<String>,
    ai: Option<String>,
    notes: Option<String>,
    supplier_id: Option<i64>,
) -> Result<i64, String> {
    supplier_service::save_supplier(
        &db, &name, contact_person, phone, email, address, rc, nif, nis, ai, notes, supplier_id,
    )
}

#[tauri::command]
pub fn delete_supplier(db: State<'_, DbState>, supplier_id: i64) -> Result<(), String> {
    supplier_service::delete_supplier(&db, supplier_id)
}

#[tauri::command]
pub fn create_purchase(db: State<'_, DbState>, input: CreatePurchaseInput) -> Result<String, String> {
    purchase_service::create_purchase(&db, input)
}

#[tauri::command]
pub fn list_purchases(db: State<'_, DbState>) -> Result<Vec<Purchase>, String> {
    purchase_service::list_purchases(&db)
}

// Expenses
#[tauri::command]
pub fn add_expense(
    db: State<'_, DbState>,
    category_id: i64,
    amount: i64,
    payment_method: String,
    session_id: Option<i64>,
    user_id: i64,
    recipient: Option<String>,
    receipt_reference: Option<String>,
    notes: Option<String>,
) -> Result<String, String> {
    expense_service::add_expense(
        &db, category_id, amount, &payment_method, session_id, user_id, recipient, receipt_reference, notes,
    )
}

#[tauri::command]
pub fn list_expenses(db: State<'_, DbState>) -> Result<Vec<Expense>, String> {
    expense_service::list_expenses(&db)
}

#[tauri::command]
pub fn delete_expense(db: State<'_, DbState>, expense_id: i64) -> Result<(), String> {
    expense_service::delete_expense(&db, expense_id)
}

// Employees & Payroll
#[tauri::command]
pub fn list_employees(db: State<'_, DbState>) -> Result<Vec<Employee>, String> {
    employee_service::list_employees(&db)
}

#[tauri::command]
pub fn save_employee(
    db: State<'_, DbState>,
    code: String,
    name: String,
    phone: Option<String>,
    email: Option<String>,
    national_id: Option<String>,
    job_title: String,
    base_salary: i64,
    salary_type: String,
    salary_start_date: Option<String>,
    hire_date: String,
    notes: Option<String>,
    employee_id: Option<i64>,
) -> Result<i64, String> {
    employee_service::save_employee(
        &db, &code, &name, phone, email, national_id, &job_title, base_salary, &salary_type, salary_start_date, &hire_date, notes, employee_id,
    )
}

#[tauri::command]
pub fn delete_employee(db: State<'_, DbState>, employee_id: i64) -> Result<(), String> {
    employee_service::delete_employee(&db, employee_id)
}

#[tauri::command]
pub fn list_payrolls(db: State<'_, DbState>) -> Result<Vec<Payroll>, String> {
    payroll_service::list_payrolls(&db)
}

// Settings & Activation
#[tauri::command]
pub fn get_all_settings(db: State<'_, DbState>) -> Result<HashMap<String, String>, String> {
    settings_service::get_all_settings(&db)
}

#[tauri::command]
pub fn set_setting(db: State<'_, DbState>, key: String, value: String) -> Result<(), String> {
    settings_service::set_setting(&db, &key, &value)
}

#[tauri::command]
pub fn set_multiple_settings(db: State<'_, DbState>, settings: HashMap<String, String>) -> Result<(), String> {
    settings_service::set_multiple_settings(&db, settings)
}

#[tauri::command]
pub fn get_hwid() -> String {
    settings_service::get_hwid()
}

#[tauri::command]
pub fn verify_license(db: State<'_, DbState>, code: String) -> Result<bool, String> {
    settings_service::verify_license(&db, &code)
}

#[tauri::command]
pub fn factory_reset(db: State<'_, DbState>, reset_type: String) -> Result<(), String> {
    settings_service::factory_reset(&db, &reset_type)
}

// Scale Integration (ACLAS Native SDK)
#[tauri::command]
pub fn test_scale_connection(ip: String, port: u32, protocol_type: u32) -> Result<String, String> {
    scale_service::test_scale_connection(&ip, port, protocol_type)
}

#[tauri::command]
pub fn upload_product_to_scale(
    db: State<'_, DbState>,
    product_id: i64,
    ip: String,
    port: u32,
    protocol_type: u32,
    default_dept: i64,
    default_barcode_type: i64,
    user_name: Option<String>,
) -> Result<usize, String> {
    let products = product_service::search_products(&db, "", None, "all")?;
    let target: Vec<Product> = products.into_iter().filter(|p| p.id == product_id).collect();
    if target.is_empty() {
        return Err("Product not found".to_string());
    }
    scale_service::upload_products_to_scale(
        &db, &target, &ip, port, protocol_type, default_dept, default_barcode_type, user_name,
    )
}

#[tauri::command]
pub fn upload_all_scalable_to_scale(
    db: State<'_, DbState>,
    ip: String,
    port: u32,
    protocol_type: u32,
    default_dept: i64,
    default_barcode_type: i64,
    user_name: Option<String>,
) -> Result<usize, String> {
    let products = product_service::search_products(&db, "", None, "all")?;
    let target: Vec<Product> = products.into_iter().filter(|p| p.is_scalable).collect();
    if target.is_empty() {
        return Err("No scalable products found to synchronize".to_string());
    }
    scale_service::upload_products_to_scale(
        &db, &target, &ip, port, protocol_type, default_dept, default_barcode_type, user_name,
    )
}

#[tauri::command]
pub fn fetch_products_from_scale(
    db: State<'_, DbState>,
    ip: String,
    port: u32,
    protocol_type: u32,
    user_name: Option<String>,
) -> Result<usize, String> {
    scale_service::fetch_products_from_scale(&db, &ip, port, protocol_type, user_name)
}

#[tauri::command]
pub fn get_scale_sync_logs(db: State<'_, DbState>) -> Result<Vec<crate::models::ScaleSyncLog>, String> {
    scale_service::get_sync_logs(&db)
}

// Direct Serial Cash Drawer
#[tauri::command]
pub fn open_serial_cash_drawer(com_port: u32, baud_rate: u32) -> Result<String, String> {
    drawer_service::open_serial_cash_drawer(com_port, baud_rate)
}

// Additional commands

#[tauri::command]
pub fn delete_sale(db: State<'_, DbState>, sale_id: i64) -> Result<(), String> {
    sales_service::delete_sale(&db, sale_id)
}

#[tauri::command]
pub fn verify_admin_password(db: State<'_, DbState>, password: String) -> Result<bool, String> {
    crate::auth::verify_admin_password(&db, &password)
}

#[tauri::command]
pub fn save_unit(
    db: State<'_, DbState>,
    name: String,
    short_name: String,
    allow_decimals: bool,
    unit_id: Option<i64>,
) -> Result<i64, String> {
    product_service::save_unit(&db, &name, &short_name, allow_decimals, unit_id)
}

#[tauri::command]
pub fn backup_database(destination_path: String) -> Result<String, String> {
    settings_service::backup_database(&destination_path)
}

#[tauri::command]
pub fn restore_database(source_backup_path: String) -> Result<String, String> {
    settings_service::restore_database(&source_backup_path)
}

#[tauri::command]
pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// User & Role Management Commands
#[tauri::command]
pub fn get_all_users(db: State<'_, DbState>) -> Result<Vec<UserAccount>, String> {
    user_service::get_all_users(&db)
}

#[tauri::command]
pub fn get_all_roles(db: State<'_, DbState>) -> Result<Vec<Role>, String> {
    user_service::get_all_roles(&db)
}

#[tauri::command]
pub fn create_user(
    db: State<'_, DbState>,
    username: String,
    display_name: String,
    password: String,
    role_id: Option<i64>,
    max_discount_percent: f64,
) -> Result<i64, String> {
    user_service::create_user(&db, &username, &display_name, &password, role_id, max_discount_percent)
}

#[tauri::command]
pub fn update_user(
    db: State<'_, DbState>,
    user_id: i64,
    username: String,
    display_name: String,
    role_id: Option<i64>,
    max_discount_percent: f64,
    is_active: bool,
    new_password: Option<String>,
) -> Result<(), String> {
    user_service::update_user(
        &db,
        user_id,
        &username,
        &display_name,
        role_id,
        max_discount_percent,
        is_active,
        new_password,
    )
}

#[tauri::command]
pub fn delete_user(db: State<'_, DbState>, user_id: i64) -> Result<(), String> {
    user_service::delete_user(&db, user_id)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AppUpdateResult {
    pub has_update: bool,
    pub current_version: String,
    pub latest_version: String,
    pub tag_name: String,
    pub release_name: String,
    pub release_notes: String,
    pub release_url: String,
    pub download_url: String,
    pub published_at: String,
}

#[tauri::command]
pub async fn check_github_update(app_handle: tauri::AppHandle) -> Result<AppUpdateResult, String> {
    let current_version = app_handle.package_info().version.to_string();

    let client = reqwest::Client::builder()
        .user_agent("TitaouPOS-Desktop")
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    let res = client
        .get("https://api.github.com/repos/titaou-bedreddine/TitaouPosT/releases")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
        .map_err(|e| format!("Failed to reach GitHub: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("GitHub API returned HTTP {}", res.status()));
    }

    let releases: Vec<serde_json::Value> = res
        .json()
        .await
        .map_err(|e| format!("Failed to parse GitHub response: {}", e))?;

    if releases.is_empty() {
        return Ok(AppUpdateResult {
            has_update: false,
            current_version: current_version.clone(),
            latest_version: current_version.clone(),
            tag_name: format!("v{}", current_version),
            release_name: "No releases found".to_string(),
            release_notes: "".to_string(),
            release_url: "https://github.com/titaou-bedreddine/TitaouPosT/releases".to_string(),
            download_url: "".to_string(),
            published_at: "".to_string(),
        });
    }

    let latest = &releases[0];
    let tag_name = latest["tag_name"].as_str().unwrap_or("").trim().to_string();
    let clean_latest = tag_name.trim_start_matches('v').trim();
    let clean_current = current_version.trim_start_matches('v').trim();

    let release_name = latest["name"].as_str().unwrap_or(&tag_name).to_string();
    let release_notes = latest["body"].as_str().unwrap_or("").to_string();
    let release_url = latest["html_url"]
        .as_str()
        .unwrap_or("https://github.com/titaou-bedreddine/TitaouPosT/releases")
        .to_string();
    let published_at = latest["published_at"].as_str().unwrap_or("").to_string();

    let mut download_url = release_url.clone();
    if let Some(assets) = latest["assets"].as_array() {
        if let Some(asset) = assets.iter().find(|a| {
            let name = a["name"].as_str().unwrap_or("");
            name.ends_with(".exe") || name.ends_with(".msi")
        }) {
            if let Some(browser_url) = asset["browser_download_url"].as_str() {
                download_url = browser_url.to_string();
            }
        }
    }

    let has_update = clean_latest != clean_current && !tag_name.is_empty();

    Ok(AppUpdateResult {
        has_update,
        current_version,
        latest_version: clean_latest.to_string(),
        tag_name,
        release_name,
        release_notes,
        release_url,
        download_url,
        published_at,
    })
}