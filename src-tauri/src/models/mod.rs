use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub display_name: String,
    pub role_id: Option<i64>,
    pub role_name: Option<String>,
    pub max_discount_percent: f64,
    pub is_active: bool,
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub name_ar: String,
    pub name_fr: String,
    pub name_en: String,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Unit {
    pub id: i64,
    pub name: String,
    pub short_name: String,
    pub allow_decimals: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductBarcode {
    pub id: Option<i64>,
    pub barcode: String,
    pub is_primary: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: i64,
    pub sku: Option<String>,
    pub name_ar: String,
    pub name_fr: String,
    pub name_en: String,
    pub category_id: Option<i64>,
    pub category_name: Option<String>,
    pub unit_id: Option<i64>,
    pub unit_name: Option<String>,
    pub purchase_price: i64,
    pub sale_price: i64,
    pub min_sale_price: i64,
    pub tax_rate: i64,
    pub current_stock: f64,
    pub min_stock: f64,
    pub max_stock: Option<f64>,
    pub image_path: Option<String>,
    pub is_bundle: bool,
    pub is_active: bool,
    pub barcodes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductBundleItemInput {
    pub component_product_id: i64,
    pub quantity: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductInput {
    pub sku: Option<String>,
    pub name_ar: String,
    pub name_fr: String,
    pub name_en: String,
    pub category_id: Option<i64>,
    pub unit_id: Option<i64>,
    pub purchase_price: i64,
    pub sale_price: i64,
    pub min_sale_price: i64,
    pub tax_rate: i64,
    pub current_stock: f64,
    pub min_stock: f64,
    pub image_path: Option<String>,
    pub is_bundle: bool,
    pub barcodes: Vec<String>,
    pub bundle_items: Option<Vec<ProductBundleItemInput>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CashSession {
    pub id: i64,
    pub register_id: i64,
    pub user_id: i64,
    pub opened_at: String,
    pub closed_at: Option<String>,
    pub opening_amount: i64,
    pub expected_cash: i64,
    pub actual_cash: Option<i64>,
    pub difference: Option<i64>,
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CashMovement {
    pub id: i64,
    pub session_id: i64,
    pub user_id: i64,
    pub type_name: String,
    pub amount: i64,
    pub reason: Option<String>,
    pub created_at: String,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CartItem {
    pub product_id: i64,
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub name_ar: String,
    pub name_fr: String,
    pub name_en: String,
    pub image_path: Option<String>,
    pub unit_price: i64,
    pub quantity: f64,
    pub discount_amount: i64,
    pub tax_amount: i64,
    pub total_price: i64,
    pub is_refund: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SalePaymentInput {
    pub payment_method: String, // 'cash', 'tpe', 'credit'
    pub amount: i64,
    pub reference_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateSaleInput {
    pub session_id: i64,
    pub user_id: i64,
    pub customer_id: Option<i64>,
    pub items: Vec<CartItem>,
    pub subtotal: i64,
    pub discount_amount: i64,
    pub discount_percentage: f64,
    pub discount_reason: Option<String>,
    pub tax_amount: i64,
    pub total_amount: i64,
    pub paid_amount: i64,
    pub change_amount: i64,
    pub payments: Vec<SalePaymentInput>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sale {
    pub id: i64,
    pub sale_number: String,
    pub session_id: Option<i64>,
    pub user_id: i64,
    pub user_name: Option<String>,
    pub customer_id: Option<i64>,
    pub customer_name: Option<String>,
    pub subtotal: i64,
    pub discount_amount: i64,
    pub tax_amount: i64,
    pub total_amount: i64,
    pub paid_amount: i64,
    pub change_amount: i64,
    pub payment_status: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Customer {
    pub id: i64,
    pub name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub tax_number: Option<String>,
    pub balance: i64,
    pub max_credit: i64,
    pub notes: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Expense {
    pub id: i64,
    pub expense_number: String,
    pub category_id: i64,
    pub category_name: Option<String>,
    pub amount: i64,
    pub payment_method: String,
    pub session_id: Option<i64>,
    pub user_id: i64,
    pub recipient: Option<String>,
    pub receipt_reference: Option<String>,
    pub date: String,
    pub notes: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Employee {
    pub id: i64,
    pub employee_code: String,
    pub full_name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub national_id: Option<String>,
    pub job_title: String,
    pub base_salary: i64,
    pub salary_type: String,
    pub hire_date: String,
    pub is_active: bool,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payroll {
    pub id: i64,
    pub payroll_number: String,
    pub employee_id: i64,
    pub employee_name: Option<String>,
    pub period_month: i64,
    pub period_year: i64,
    pub base_salary: i64,
    pub bonuses: i64,
    pub allowances: i64,
    pub deductions: i64,
    pub advances_deducted: i64,
    pub net_salary: i64,
    pub payment_status: String,
    pub payment_method: Option<String>,
    pub paid_at: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DashboardStats {
    pub today_sales: i64,
    pub today_transactions_count: i64,
    pub today_profit: i64,
    pub low_stock_count: i64,
    pub out_of_stock_count: i64,
    pub active_cash_expected: i64,
    pub today_expenses: i64,
}
