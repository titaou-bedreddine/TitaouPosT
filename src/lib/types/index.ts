export interface User {
  id: number;
  username: string;
  display_name: string;
  role_id?: number;
  role_name?: string;
  max_discount_percent: number;
  is_active: boolean;
  permissions: string[];
}

export interface Category {
  id: number;
  parent_id?: number;
  name_ar: string;
  name_fr: string;
  name_en: string;
  is_active: boolean;
}

export interface Unit {
  id: number;
  name: string;
  short_name: string;
  allow_decimals: boolean;
}

export interface Product {
  id: number;
  sku?: string;
  name_ar: string;
  name_fr: string;
  name_en: string;
  category_id?: number;
  category_name?: string;
  unit_id?: number;
  unit_name?: string;
  purchase_price: number;
  sale_price: number;
  min_sale_price: number;
  tax_rate: number;
  current_stock: number;
  min_stock: number;
  max_stock?: number;
  image_path?: string;
  is_bundle: boolean;
  is_active: boolean;
  barcodes: string[];
}

export interface CartItem {
  product_id: number;
  sku?: string;
  barcode?: string;
  name_ar: string;
  name_fr: string;
  name_en: string;
  image_path?: string;
  unit_price: number;
  quantity: number;
  discount_amount: number;
  tax_amount: number;
  total_price: number;
  is_refund: boolean;
}

export interface CashSession {
  id: number;
  register_id: number;
  user_id: number;
  opened_at: string;
  closed_at?: string;
  opening_amount: number;
  expected_cash: number;
  actual_cash?: number;
  difference?: number;
  status: 'open' | 'closed';
  notes?: string;
}

export interface CashMovement {
  id: number;
  session_id: number;
  user_id: number;
  type_name: string;
  amount: number;
  reason?: string;
  created_at: string;
  notes?: string;
}

export interface Sale {
  id: number;
  sale_number: string;
  session_id?: number;
  user_id: number;
  user_name?: string;
  customer_id?: number;
  customer_name?: string;
  subtotal: number;
  discount_amount: number;
  tax_amount: number;
  total_amount: number;
  paid_amount: number;
  change_amount: number;
  payment_status: string;
  status: string;
  created_at: string;
}

export interface Expense {
  id: number;
  expense_number: string;
  category_id: number;
  category_name?: string;
  amount: number;
  payment_method: string;
  session_id?: number;
  user_id: number;
  recipient?: string;
  receipt_reference?: string;
  date: string;
  notes?: string;
  created_at: string;
}

export interface Employee {
  id: number;
  employee_code: string;
  full_name: string;
  phone?: string;
  email?: string;
  national_id?: string;
  job_title: string;
  base_salary: number;
  salary_type: string;
  hire_date: string;
  is_active: boolean;
  notes?: string;
}

export interface Payroll {
  id: number;
  payroll_number: string;
  employee_id: number;
  employee_name?: string;
  period_month: number;
  period_year: number;
  base_salary: number;
  bonuses: number;
  allowances: number;
  deductions: number;
  advances_deducted: number;
  net_salary: number;
  payment_status: string;
  payment_method?: string;
  paid_at?: string;
  notes?: string;
}

export interface DashboardStats {
  today_sales: number;
  today_transactions_count: number;
  today_profit: number;
  low_stock_count: number;
  out_of_stock_count: number;
  active_cash_expected: number;
  today_expenses: number;
}