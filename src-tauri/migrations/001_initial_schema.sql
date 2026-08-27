-- TitaouPosT Database Schema Migration 001

PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;

-- 1. Security, Users, Roles & Permissions
CREATE TABLE IF NOT EXISTS roles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    is_system BOOLEAN DEFAULT 0
);

CREATE TABLE IF NOT EXISTS permissions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    code TEXT NOT NULL UNIQUE,
    category TEXT NOT NULL,
    description TEXT
);

CREATE TABLE IF NOT EXISTS role_permissions (
    role_id INTEGER REFERENCES roles(id) ON DELETE CASCADE,
    permission_id INTEGER REFERENCES permissions(id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_id)
);

CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    pin_hash TEXT,
    role_id INTEGER REFERENCES roles(id),
    max_discount_percent REAL DEFAULT 100.0,
    is_active BOOLEAN DEFAULT 1,
    last_login DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS audit_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER REFERENCES users(id),
    action TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id INTEGER,
    description TEXT NOT NULL,
    metadata_json TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 2. Cash Sessions & Register Management (صندوق / Fond de Caisse)
CREATE TABLE IF NOT EXISTS registers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    identifier TEXT NOT NULL UNIQUE,
    is_active BOOLEAN DEFAULT 1
);

CREATE TABLE IF NOT EXISTS cash_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    register_id INTEGER REFERENCES registers(id),
    user_id INTEGER REFERENCES users(id),
    opened_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    closed_at DATETIME,
    opening_amount INTEGER NOT NULL DEFAULT 0,
    expected_cash INTEGER DEFAULT 0,
    actual_cash INTEGER,
    difference INTEGER,
    status TEXT CHECK(status IN ('open', 'closed')) DEFAULT 'open',
    notes TEXT
);

CREATE TABLE IF NOT EXISTS cash_movements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER REFERENCES cash_sessions(id),
    user_id INTEGER REFERENCES users(id),
    type TEXT CHECK(type IN ('opening_balance', 'cash_sale', 'cash_refund', 'cash_in', 'cash_out', 'expense_payment', 'salary_payment', 'customer_debt_payment', 'supplier_debt_payment', 'closing_adjustment')) NOT NULL,
    amount INTEGER NOT NULL,
    reason TEXT,
    reference_type TEXT,
    reference_id INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    notes TEXT
);

-- 3. Product Catalog, Categories, Multi-Barcodes & Bundles
CREATE TABLE IF NOT EXISTS categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    parent_id INTEGER REFERENCES categories(id),
    name_ar TEXT NOT NULL,
    name_fr TEXT NOT NULL,
    name_en TEXT NOT NULL,
    color TEXT DEFAULT '#0284c7',
    is_active BOOLEAN DEFAULT 1
);

CREATE TABLE IF NOT EXISTS units (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    short_name TEXT NOT NULL,
    allow_decimals BOOLEAN DEFAULT 0
);

CREATE TABLE IF NOT EXISTS products (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sku TEXT UNIQUE,
    name_ar TEXT NOT NULL,
    name_fr TEXT NOT NULL,
    name_en TEXT NOT NULL,
    category_id INTEGER REFERENCES categories(id) ON DELETE SET NULL,
    unit_id INTEGER REFERENCES units(id) ON DELETE SET NULL,
    purchase_price INTEGER NOT NULL DEFAULT 0,
    sale_price INTEGER NOT NULL DEFAULT 0,
    min_sale_price INTEGER NOT NULL DEFAULT 0,
    tax_rate INTEGER DEFAULT 0,
    current_stock REAL DEFAULT 0,
    min_stock REAL DEFAULT 5,
    max_stock REAL,
    image_path TEXT,
    is_bundle BOOLEAN DEFAULT 0,
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS product_barcodes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    product_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    barcode TEXT NOT NULL UNIQUE,
    is_primary BOOLEAN DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS product_bundle_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    bundle_product_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    component_product_id INTEGER NOT NULL REFERENCES products(id),
    quantity REAL NOT NULL
);

CREATE TABLE IF NOT EXISTS inventory_movements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    product_id INTEGER NOT NULL REFERENCES products(id),
    quantity REAL NOT NULL,
    type TEXT CHECK(type IN ('purchase', 'sale', 'sale_refund', 'adjustment_inc', 'adjustment_dec', 'bundle_sale', 'bundle_refund')) NOT NULL,
    reference_type TEXT,
    reference_id INTEGER,
    user_id INTEGER REFERENCES users(id),
    cost_at_time INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    notes TEXT
);

-- 4. Customers & Suppliers
CREATE TABLE IF NOT EXISTS customers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    phone TEXT,
    email TEXT,
    address TEXT,
    rc TEXT,
    nif TEXT,
    nis TEXT,
    ai TEXT,
    qr_code TEXT,
    balance INTEGER DEFAULT 0,
    initial_debt INTEGER DEFAULT 0,
    notes TEXT,
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS customer_debt_payments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    customer_id INTEGER NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    amount INTEGER NOT NULL,
    payment_method TEXT CHECK(payment_method IN ('cash', 'bank_transfer', 'cheque')) DEFAULT 'cash',
    reference TEXT,
    session_id INTEGER REFERENCES cash_sessions(id),
    user_id INTEGER REFERENCES users(id),
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS suppliers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    contact_person TEXT,
    phone TEXT,
    email TEXT,
    address TEXT,
    rc TEXT,
    nif TEXT,
    nis TEXT,
    ai TEXT,
    qr_code TEXT,
    balance INTEGER DEFAULT 0,
    notes TEXT,
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS supplier_debt_payments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    supplier_id INTEGER NOT NULL REFERENCES suppliers(id) ON DELETE CASCADE,
    amount INTEGER NOT NULL,
    payment_method TEXT CHECK(payment_method IN ('cash', 'bank_transfer', 'cheque')) DEFAULT 'cash',
    reference TEXT,
    session_id INTEGER REFERENCES cash_sessions(id),
    user_id INTEGER REFERENCES users(id),
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 5. Purchasing & Purchase Invoices
CREATE TABLE IF NOT EXISTS purchases (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    invoice_number TEXT NOT NULL UNIQUE,
    supplier_id INTEGER REFERENCES suppliers(id),
    user_id INTEGER REFERENCES users(id),
    date DATE NOT NULL,
    subtotal INTEGER NOT NULL,
    discount INTEGER DEFAULT 0,
    tax INTEGER DEFAULT 0,
    total INTEGER NOT NULL,
    paid_amount INTEGER NOT NULL,
    payment_method TEXT CHECK(payment_method IN ('cash', 'tpe', 'bank_transfer', 'credit')) DEFAULT 'cash',
    status TEXT CHECK(status IN ('received', 'pending', 'cancelled')) DEFAULT 'received',
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS purchase_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    purchase_id INTEGER NOT NULL REFERENCES purchases(id) ON DELETE CASCADE,
    product_id INTEGER NOT NULL REFERENCES products(id),
    quantity REAL NOT NULL,
    unit_cost INTEGER NOT NULL,
    discount INTEGER DEFAULT 0,
    tax INTEGER DEFAULT 0,
    total INTEGER NOT NULL
);

-- 6. Sales, POS, Held Orders & Refunds
CREATE TABLE IF NOT EXISTS sales (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sale_number TEXT NOT NULL UNIQUE,
    session_id INTEGER REFERENCES cash_sessions(id),
    user_id INTEGER REFERENCES users(id),
    customer_id INTEGER REFERENCES customers(id),
    subtotal INTEGER NOT NULL,
    discount_amount INTEGER DEFAULT 0,
    discount_percentage REAL DEFAULT 0,
    discount_reason TEXT,
    tax_amount INTEGER DEFAULT 0,
    total_amount INTEGER NOT NULL,
    paid_amount INTEGER NOT NULL,
    change_amount INTEGER DEFAULT 0,
    payment_status TEXT CHECK(payment_status IN ('paid', 'partial', 'credit')) NOT NULL,
    status TEXT CHECK(status IN ('completed', 'cancelled', 'refunded', 'partially_refunded')) DEFAULT 'completed',
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS sale_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sale_id INTEGER NOT NULL REFERENCES sales(id) ON DELETE CASCADE,
    product_id INTEGER NOT NULL REFERENCES products(id),
    quantity REAL NOT NULL,
    unit_price INTEGER NOT NULL,
    discount_amount INTEGER DEFAULT 0,
    tax_amount INTEGER DEFAULT 0,
    total_price INTEGER NOT NULL,
    is_refunded BOOLEAN DEFAULT 0,
    refunded_quantity REAL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS sale_payments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sale_id INTEGER NOT NULL REFERENCES sales(id) ON DELETE CASCADE,
    payment_method TEXT CHECK(payment_method IN ('cash', 'tpe', 'credit')) NOT NULL,
    amount INTEGER NOT NULL,
    reference_code TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS held_sales (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sale_reference TEXT NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users(id),
    customer_id INTEGER REFERENCES customers(id),
    cart_json TEXT NOT NULL,
    note TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 7. Expenses (المصاريف)
CREATE TABLE IF NOT EXISTS expense_categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name_ar TEXT NOT NULL,
    name_fr TEXT NOT NULL,
    name_en TEXT NOT NULL,
    description TEXT,
    is_active BOOLEAN DEFAULT 1
);

CREATE TABLE IF NOT EXISTS expenses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    expense_number TEXT NOT NULL UNIQUE,
    category_id INTEGER NOT NULL REFERENCES expense_categories(id),
    amount INTEGER NOT NULL,
    payment_method TEXT CHECK(payment_method IN ('cash', 'tpe', 'bank_transfer', 'other')) NOT NULL,
    session_id INTEGER REFERENCES cash_sessions(id),
    user_id INTEGER NOT NULL REFERENCES users(id),
    recipient TEXT,
    receipt_reference TEXT,
    date DATE NOT NULL,
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 8. Employees & Payrolls (رواتب الموظفين)
CREATE TABLE IF NOT EXISTS employees (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    employee_code TEXT NOT NULL UNIQUE,
    full_name TEXT NOT NULL,
    phone TEXT,
    email TEXT,
    national_id TEXT,
    job_title TEXT NOT NULL,
    base_salary INTEGER NOT NULL,
    salary_type TEXT CHECK(salary_type IN ('monthly', 'daily', 'hourly')) DEFAULT 'monthly',
    salary_start_date DATE,
    hire_date DATE NOT NULL,
    user_account_id INTEGER REFERENCES users(id),
    qr_code TEXT,
    is_active BOOLEAN DEFAULT 1,
    notes TEXT
);

CREATE TABLE IF NOT EXISTS salary_advances (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    employee_id INTEGER NOT NULL REFERENCES employees(id),
    amount INTEGER NOT NULL,
    payment_method TEXT CHECK(payment_method IN ('cash', 'bank_transfer')) NOT NULL,
    session_id INTEGER REFERENCES cash_sessions(id),
    user_id INTEGER REFERENCES users(id),
    date DATE NOT NULL,
    is_deducted BOOLEAN DEFAULT 0,
    payroll_id INTEGER,
    reason TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS payrolls (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    payroll_number TEXT NOT NULL UNIQUE,
    employee_id INTEGER NOT NULL REFERENCES employees(id),
    period_month INTEGER NOT NULL,
    period_year INTEGER NOT NULL,
    base_salary INTEGER NOT NULL,
    bonuses INTEGER DEFAULT 0,
    allowances INTEGER DEFAULT 0,
    deductions INTEGER DEFAULT 0,
    advances_deducted INTEGER DEFAULT 0,
    net_salary INTEGER NOT NULL,
    payment_status TEXT CHECK(payment_status IN ('pending', 'paid')) DEFAULT 'pending',
    payment_method TEXT CHECK(payment_method IN ('cash', 'bank_transfer', 'check')),
    session_id INTEGER REFERENCES cash_sessions(id),
    paid_at DATETIME,
    user_id INTEGER REFERENCES users(id),
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 9. Paired Android / Mobile Devices
CREATE TABLE IF NOT EXISTS paired_devices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    device_name TEXT NOT NULL,
    device_uid TEXT NOT NULL UNIQUE,
    api_token_hash TEXT NOT NULL,
    assigned_user_id INTEGER REFERENCES users(id),
    device_role TEXT CHECK(device_role IN ('pos_terminal', 'mobile_waiter', 'inventory_scanner', 'manager_view')) NOT NULL,
    is_authorized BOOLEAN DEFAULT 1,
    last_connected_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 10. Hardware, Notifications & System Settings
CREATE TABLE IF NOT EXISTS printer_settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    receipt_printer_name TEXT,
    label_printer_name TEXT,
    receipt_paper_width TEXT DEFAULT '80mm',
    auto_print_sale BOOLEAN DEFAULT 1,
    print_logo BOOLEAN DEFAULT 0,
    header_text TEXT,
    footer_text TEXT
);

CREATE TABLE IF NOT EXISTS notification_queue (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    channel TEXT CHECK(channel IN ('telegram', 'gmail')) NOT NULL,
    event_type TEXT NOT NULL,
    payload_json TEXT NOT NULL,
    status TEXT CHECK(status IN ('pending', 'sent', 'failed')) DEFAULT 'pending',
    attempts INTEGER DEFAULT 0,
    last_attempt_at DATETIME,
    error_message TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS backups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_name TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    type TEXT CHECK(type IN ('local_auto', 'local_manual', 'cloud_google_drive')) NOT NULL,
    status TEXT NOT NULL,
    hash TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS app_settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Essential Performance Indexes
CREATE INDEX IF NOT EXISTS idx_barcodes_lookup ON product_barcodes(barcode);
CREATE INDEX IF NOT EXISTS idx_products_sku ON products(sku);
CREATE INDEX IF NOT EXISTS idx_products_name_ar ON products(name_ar);
CREATE INDEX IF NOT EXISTS idx_products_name_en ON products(name_en);
CREATE INDEX IF NOT EXISTS idx_sales_date ON sales(created_at);
CREATE INDEX IF NOT EXISTS idx_sales_session ON sales(session_id);
CREATE INDEX IF NOT EXISTS idx_inventory_product ON inventory_movements(product_id);
CREATE INDEX IF NOT EXISTS idx_cash_movements_session ON cash_movements(session_id);