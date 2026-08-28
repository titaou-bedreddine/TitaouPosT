use crate::auth::authenticate_user;
use crate::database::DbState;
use crate::models::{
    CartItem, Category, Customer, CustomerPaymentInput, DashboardStats, Employee, Expense, HeldSale,
    Payroll, Product, ProductInput, Purchase, CreatePurchaseInput, Sale, Supplier, Unit, User,
    CashMovement, CashSession, CreateSaleInput,
};
use crate::services::{
    cash_service, customer_service, dashboard_service, employee_service, expense_service,
    payroll_service, product_service, purchase_service, sales_service, settings_service,
    supplier_service,
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
    user_id: i64,
    customer_id: Option<i64>,
    cart_json: String,
    note: Option<String>,
) -> Result<i64, String> {
    sales_service::hold_sale(&db, user_id, customer_id, &cart_json, note)
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