-- Seed Roles
INSERT OR IGNORE INTO roles (id, name, description, is_system) VALUES 
(1, 'Administrator', 'Full system access', 1),
(2, 'Manager', 'Store operations & reports management', 1),
(3, 'Cashier', 'POS sales & basic cashier functions', 1);

-- Seed Permissions
INSERT OR IGNORE INTO permissions (code, category, description) VALUES
('dashboard.view', 'dashboard', 'View main dashboard'),
('sales.view', 'sales', 'View sales list'),
('sales.create', 'sales', 'Create POS sales'),
('sales.refund', 'sales', 'Process refunds'),
('sales.discount', 'sales', 'Apply discounts'),
('cash.open', 'cash', 'Open cash session'),
('cash.close', 'cash', 'Close cash session'),
('cash.in', 'cash', 'Record Cash In'),
('cash.out', 'cash', 'Record Cash Out'),
('inventory.view', 'inventory', 'View products and stock'),
('inventory.manage', 'inventory', 'Create and edit products'),
('inventory.adjust', 'inventory', 'Adjust stock levels'),
('purchases.manage', 'purchases', 'Manage supplier purchases'),
('expenses.manage', 'expenses', 'Manage expenses'),
('payroll.manage', 'payroll', 'Manage employees & payroll'),
('reports.view', 'reports', 'View analytical reports'),
('users.manage', 'users', 'Manage user accounts'),
('settings.manage', 'settings', 'Manage system settings'),
('backup.manage', 'backup', 'Manage local and cloud backups');

-- Assign all permissions to Administrator
INSERT OR IGNORE INTO role_permissions (role_id, permission_id)
SELECT 1, id FROM permissions;

-- Assign Cashier permissions
INSERT OR IGNORE INTO role_permissions (role_id, permission_id)
SELECT 3, id FROM permissions WHERE code IN ('sales.view', 'sales.create', 'cash.open', 'cash.close', 'inventory.view');

-- Seed Default Register
INSERT OR IGNORE INTO registers (id, name, identifier, is_active) VALUES
(1, 'Main Register (Caisse 01)', 'REG-01', 1);

-- Seed Default Units
INSERT OR IGNORE INTO units (id, name, short_name, allow_decimals) VALUES
(1, 'Piece / Pièce / قطعة', 'pcs', 0),
(2, 'Kilogram / Kilogramme / كغ', 'kg', 1),
(3, 'Liter / Litre / لتر', 'L', 1),
(4, 'Box / Boîte / علبة', 'box', 0),
(5, 'Meter / Mètre / متر', 'm', 1);

-- Seed Categories with Colors
INSERT OR IGNORE INTO categories (id, parent_id, name_ar, name_fr, name_en, color, is_active) VALUES
(1, NULL, 'المشروبات والعصائر', 'Boissons & Jus', 'Beverages & Juices', '#0284c7', 1),
(2, NULL, 'المواد الغذائية', 'Alimentation générale', 'Groceries', '#10b981', 1),
(3, NULL, 'الحلويات والشوكولاطة', 'Confiserie & Chocolat', 'Sweets & Chocolate', '#f59e0b', 1),
(4, NULL, 'منتجات الألبان والأجبان', 'Produits laitiers & Fromage', 'Dairy & Cheese', '#8b5cf6', 1),
(5, NULL, 'مواد التنظيف والعناية', 'Entretien & Hygiène', 'Cleaning & Care', '#ec4899', 1);

-- Seed Sample Products
INSERT OR IGNORE INTO products (id, sku, name_ar, name_fr, name_en, category_id, unit_id, purchase_price, sale_price, min_sale_price, tax_rate, current_stock, min_stock, is_bundle, is_active) VALUES
(1, 'PRD-001', 'مياه معدنية لالة خديجة 1.5 لتر', 'Eau Minérale Lalla Khedidja 1.5L', 'Mineral Water 1.5L', 1, 1, 35, 45, 40, 0, 120, 10, 0, 1),
(2, 'PRD-002', 'مشروب غازي كوكاكولا 330 مل', 'Boisson Gazeuse Coca-Cola 330ml', 'Coca-Cola Can 330ml', 1, 1, 90, 120, 110, 0, 75, 12, 0, 1),
(3, 'PRD-003', 'حليب معقم كانديا 1 لتر', 'Lait UHT Candia Silhouette 1L', 'UHT Milk 1L', 4, 1, 110, 135, 130, 0, 50, 8, 0, 1),
(4, 'PRD-004', 'زيت المائدة إيليو 5 لتر', 'Huile de Table Elio 5L', 'Cooking Oil 5L', 2, 1, 580, 650, 630, 0, 24, 5, 0, 1),
(5, 'PRD-005', 'شوكولاطة ماكسون بالبندق 100غ', 'Chocolat Maxon Noisette 100g', 'Maxon Chocolate 100g', 3, 1, 180, 240, 220, 0, 40, 6, 0, 1),
(6, 'PRD-006', 'قهوة فاميكو مرحية 250غ', 'Café Moulu Famico 250g', 'Ground Coffee 250g', 2, 1, 220, 280, 260, 0, 3, 5, 0, 1),
(7, 'PRD-007', 'منظف أواني إيزيس 1 لتر', 'Liquide Vaisselle ISIS 1L', 'Dish Soap ISIS 1L', 5, 1, 160, 210, 200, 0, 0, 4, 0, 1);

-- Seed Barcodes
INSERT OR IGNORE INTO product_barcodes (product_id, barcode, is_primary) VALUES
(1, '613000100101', 1),
(1, '613000100102', 0),
(2, '5449000000996', 1),
(3, '3176571992015', 1),
(4, '6130456000055', 1),
(5, '6130987000112', 1),
(6, '6130123456789', 1),
(7, '6130765432100', 1);

-- Seed Customers
INSERT OR IGNORE INTO customers (id, name, phone, email, address, rc, nif, nis, ai, qr_code, balance, initial_debt, notes, is_active) VALUES
(1, 'زبون عادي (Walk-in Customer)', '', '', '', '', '', '', '', 'CUST-001', 0, 0, 'Comptoir', 1),
(2, 'محمد بوعلام (Ets Boualem)', '0555 12 34 56', 'boualem@example.dz', 'Alger Centre', '16/00-1234567B18', '001816012345678', '1990160100123', '16012345678', 'CUST-002', 12500, 12500, 'Client fidèle', 1),
(3, 'شركة لومينا الرقمية (Lumina SARL)', '021 66 77 88', 'contact@lumina.dz', 'Bab Ezzouar, Algiers', '16/00-9876543B20', '002016098765432', '1995160100456', '16098765432', 'CUST-003', 0, 0, 'Facturation mensuelle', 1);

-- Seed Suppliers
INSERT OR IGNORE INTO suppliers (id, name, contact_person, phone, email, address, rc, nif, nis, ai, qr_code, balance, notes, is_active) VALUES
(1, 'مجمع كانديا للألبان (Candia Algérie)', 'Karim Mehdi', '021 88 99 00', 'orders@candia.dz', 'Zone Industrielle Rouiba', '16/00-1112223B15', '001516011122233', '1985160100789', '16011122233', 'SUPP-001', 45000, 'Fournisseur Lait', 1),
(2, 'شركة كوكاكولا الجزائر (Coca-Cola Bottling)', 'Samir Haddad', '0550 44 55 66', 'sales@coca-algerie.dz', 'Zone Industrielle Oued Smar', '16/00-3334445B16', '001616033344455', '1988160100321', '16033344455', 'SUPP-002', 18000, 'Fournisseur Boissons', 1);

-- Seed Expense Categories
INSERT OR IGNORE INTO expense_categories (id, name_ar, name_fr, name_en, is_active) VALUES
(1, 'إيجار المحل', 'Loyer du local', 'Store Rent', 1),
(2, 'الكهرباء والماء والغاز', 'Électricité, Eau & Gaz', 'Utilities', 1),
(3, 'الصيانة والعتاد', 'Maintenance & Réparations', 'Maintenance', 1),
(4, 'النقل والشحن والتوصيل', 'Transport & Livraison', 'Transport & Shipping', 1),
(5, 'لوازم التغليف والمكتب', 'Emballage & Fournitures', 'Store Supplies', 1),
(6, 'مصاريف تسويق وضيافة', 'Marketing & Accueil', 'Marketing & Hospitality', 1);

-- Seed Sample Expenses
INSERT OR IGNORE INTO expenses (id, expense_number, category_id, amount, payment_method, session_id, user_id, recipient, receipt_reference, date, notes) VALUES
(1, 'EXP-20260827-01', 2, 4500, 'cash', 1, 1, 'Sonelgaz', 'FACT-SON-9872', CURRENT_DATE, 'فاتورة الكهرباء لشهر أوت'),
(2, 'EXP-20260827-02', 5, 2300, 'cash', 1, 1, 'Papeterie Moderne', 'REC-4412', CURRENT_DATE, 'أكياس تعبئة ورولات حرارية');

-- Seed Employees
INSERT OR IGNORE INTO employees (id, employee_code, full_name, phone, email, national_id, job_title, base_salary, salary_type, salary_start_date, hire_date, qr_code, is_active, notes) VALUES
(1, 'EMP-01', 'أحمد بن علي (Ahmed Benali)', '0555 99 88 77', 'ahmed@pos.dz', '10987654321', 'مسؤول الصندوق (Head Cashier)', 45000, 'monthly', '2026-01-01', '2026-01-01', 'EMP_QR_01', 1, 'دوام كامل'),
(2, 'EMP-02', 'فاطمة الزهراء (Fatima Zohra)', '0661 22 33 44', 'fatima@pos.dz', '10876543210', 'بائعة وأمين مخزن (Sales & Stock)', 38000, 'monthly', '2026-03-01', '2026-03-01', 'EMP_QR_02', 1, 'دوام صباحي');

-- Seed Default Settings
INSERT OR IGNORE INTO app_settings (key, value) VALUES
('app_language', 'ar'),
('app_theme', 'light'),
('currency_symbol', 'DZD'),
('currency_name', 'Algerian Dinar'),
('shop_name', 'لومينا ديجيتال سيرفيس - TitaouPosT'),
('shop_description', 'Superette & Commerce Général'),
('shop_address', 'Didouche Mourad St., Algiers, Algeria'),
('shop_phone', '0555 00 11 22'),
('shop_landline', '021 77 88 99'),
('shop_email', 'contact@titaoupos.dz'),
('shop_rc', '16/00-1234567B22'),
('shop_nif', '002216012345678'),
('shop_nis', '1998160100999'),
('shop_ai', '16012345678'),
('shop_tva', '19'),
('receipt_paper_width', '80mm'),
('cash_drawer_enabled', '1'),
('skip_quick_sale_confirm', '0'),
('allow_negative_stock', '1'),
('financial_only_mode', '0'),
('require_confirm_no_stock', '1'),
('droit_de_timbre', '1'),
('barcode_label_width', '50'),
('barcode_label_height', '30'),
('barcode_label_barcode_height', '10'),
('barcode_opt_shop_name', '0'),
('barcode_opt_product_name', '1'),
('barcode_opt_price', '1'),
('barcode_opt_price_type', '1'),
('barcode_opt_barcode_number', '1'),
('barcode_opt_variants', '0'),
('barcode_opt_discount', '0'),
('active_invoice_template', 'template_1'),
('active_receipt_template', 'template_1'),
('app_hwid', '4e67912a-d4ee-49bc-9bd0-6dccc402a6e6'),
('app_license_status', 'trial');