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
    pub color: String,
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
    pub expiry_date: Option<String>,
    pub is_bundle: bool,
    pub is_active: bool,
    pub barcodes: Vec<String>,
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
    pub expiry_date: Option<String>,
    pub is_bundle: bool,
    pub barcodes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CashSession {
    pub id: i64,
    pub register_id: i64,
    pub user_id: i64,
    pub user_name: Option<String>,
    pub opened_at: String,
    pub closed_at: Option<String>,
    pub opening_amount: i64,
    pub expected_cash: i64,
    pub actual_cash: Option<i64>,
    pub difference: Option<i64>,
    pub total_sales: Option<i64>,
    pub total_expenses: Option<i64>,
    pub current_balance: Option<i64>,
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CashMovement {
    pub id: i64,
    pub session_id: i64,
    pub user_id: i64,
    pub user_name: Option<String>,
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
    pub payment_method: String,
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
    pub items: Option<Vec<CartItem>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Customer {
    pub id: i64,
    pub name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub rc: Option<String>,
    pub nif: Option<String>,
    pub nis: Option<String>,
    pub ai: Option<String>,
    pub qr_code: Option<String>,
    pub balance: i64,
    pub initial_debt: i64,
    pub total_purchases: Option<i64>,
    pub notes: Option<String>,
    pub is_active: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerPaymentInput {
    pub customer_id: i64,
    pub amount: i64,
    pub payment_method: String,
    pub reference: Option<String>,
    pub session_id: Option<i64>,
    pub user_id: i64,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Supplier {
    pub id: i64,
    pub name: String,
    pub contact_person: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub rc: Option<String>,
    pub nif: Option<String>,
    pub nis: Option<String>,
    pub ai: Option<String>,
    pub qr_code: Option<String>,
    pub balance: i64,
    pub notes: Option<String>,
    pub is_active: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PurchaseItemInput {
    pub product_id: i64,
    pub quantity: f64,
    pub unit_cost: i64,
    pub discount: i64,
    pub tax: i64,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreatePurchaseInput {
    pub invoice_number: String,
    pub supplier_id: i64,
    pub user_id: i64,
    pub date: String,
    pub subtotal: i64,
    pub discount: i64,
    pub tax: i64,
    pub total: i64,
    pub paid_amount: i64,
    pub payment_method: String,
    pub items: Vec<PurchaseItemInput>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Purchase {
    pub id: i64,
    pub invoice_number: String,
    pub supplier_id: i64,
    pub supplier_name: Option<String>,
    pub user_id: i64,
    pub user_name: Option<String>,
    pub date: String,
    pub subtotal: i64,
    pub discount: i64,
    pub tax: i64,
    pub total: i64,
    pub paid_amount: i64,
    pub payment_method: String,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: String,
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
    pub salary_start_date: Option<String>,
    pub hire_date: String,
    pub qr_code: Option<String>,
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
pub struct HeldSale {
    pub id: i64,
    pub sale_reference: String,
    pub user_id: i64,
    pub customer_id: Option<i64>,
    pub cart_json: String,
    pub note: Option<String>,
    pub created_at: String,
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
    pub returns_amount: i64,
    pub net_revenue: i64,
    pub cost_of_goods: i64,
    pub gross_profit: i64,
    pub average_basket: i64,
    pub top_products: Vec<TopProductStat>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TopProductStat {
    pub product_name: String,
    pub category_name: String,
    pub sold_qty: f64,
    pub revenue: i64,
    pub cost: i64,
    pub profit: i64,
}