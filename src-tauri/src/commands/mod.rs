use crate::auth::authenticate_user;
use crate::database::DbState;
use crate::models::{
    Category, DashboardStats, Employee, Expense, Payroll, Product, ProductInput, Sale, Unit, User,
    CashMovement, CashSession, CreateSaleInput,
};
use crate::services::{
    cash_service, dashboard_service, expense_service, payroll_service, product_service,
    sales_service,
};
use tauri::State;

#[tauri::command]
pub fn login(db: State<'_, DbState>, username: String, password: String) -> Result<Option<User>, String> {
    authenticate_user(&db, &username, &password)
}

#[tauri::command]
pub fn get_dashboard_stats(db: State<'_, DbState>) -> Result<DashboardStats, String> {
    dashboard_service::get_stats(&db)
}

#[tauri::command]
pub fn get_active_cash_session(db: State<'_, DbState>, user_id: i64) -> Result<Option<CashSession>, String> {
    cash_service::get_active_session(&db, user_id)
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
pub fn get_categories(db: State<'_, DbState>) -> Result<Vec<Category>, String> {
    product_service::get_categories(&db)
}

#[tauri::command]
pub fn get_units(db: State<'_, DbState>) -> Result<Vec<Unit>, String> {
    product_service::get_units(&db)
}

#[tauri::command]
pub fn process_sale(db: State<'_, DbState>, input: CreateSaleInput) -> Result<String, String> {
    sales_service::process_sale(&db, input)
}

#[tauri::command]
pub fn list_sales(db: State<'_, DbState>, limit: i64) -> Result<Vec<Sale>, String> {
    sales_service::list_sales(&db, limit)
}

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
        &db,
        category_id,
        amount,
        &payment_method,
        session_id,
        user_id,
        recipient,
        receipt_reference,
        notes,
    )
}

#[tauri::command]
pub fn list_expenses(db: State<'_, DbState>) -> Result<Vec<Expense>, String> {
    expense_service::list_expenses(&db)
}

#[tauri::command]
pub fn list_employees(db: State<'_, DbState>) -> Result<Vec<Employee>, String> {
    payroll_service::list_employees(&db)
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
    hire_date: String,
    notes: Option<String>,
    employee_id: Option<i64>,
) -> Result<i64, String> {
    payroll_service::save_employee(
        &db, &code, &name, phone, email, national_id, &job_title, base_salary, &salary_type, &hire_date, notes, employee_id
    )
}

#[tauri::command]
pub fn list_payrolls(db: State<'_, DbState>) -> Result<Vec<Payroll>, String> {
    payroll_service::list_payrolls(&db)
}
