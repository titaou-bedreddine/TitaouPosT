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
(1, 'Main Register (Caisse Principale)', 'REG-01', 1);

-- Seed Default Units
INSERT OR IGNORE INTO units (id, name, short_name, allow_decimals) VALUES
(1, 'Piece / Pièce / قطعة', 'pcs', 0),
(2, 'Kilogram / Kilogramme / كغ', 'kg', 1),
(3, 'Liter / Litre / لتر', 'L', 1),
(4, 'Box / Boîte / علبة', 'box', 0),
(5, 'Meter / Mètre / متر', 'm', 1);

-- Seed Default Expense Categories
INSERT OR IGNORE INTO expense_categories (id, name_ar, name_fr, name_en, is_active) VALUES
(1, 'إيجار المحل', 'Loyer', 'Rent', 1),
(2, 'الكهرباء والماء', 'Électricité & Eau', 'Utilities', 1),
(3, 'الصيانة والإصلاح', 'Maintenance & Réparations', 'Maintenance', 1),
(4, 'النقل والتوصيل', 'Transport & Livraison', 'Transport', 1),
(5, 'لوازم المحل', 'Fournitures de magasin', 'Store Supplies', 1),
(6, 'مصاريف أخرى', 'Autres dépenses', 'Miscellaneous', 1);

-- Seed Default Categories
INSERT OR IGNORE INTO categories (id, name_ar, name_fr, name_en, is_active) VALUES
(1, 'المشروبات', 'Boissons', 'Beverages', 1),
(2, 'المواد الغذائية', 'Alimentation générale', 'Groceries', 1),
(3, 'الحلويات والبسكويت', 'Confiserie & Biscuits', 'Confectionery', 1),
(4, 'منتجات الحليب', 'Produits laitiers', 'Dairy Products', 1),
(5, 'مواد التنظيف', 'Produits d''entretien', 'Cleaning Supplies', 1);

-- Default Application Settings
INSERT OR IGNORE INTO app_settings (key, value) VALUES
('app_language', 'ar'),
('app_theme', 'light'),
('currency_symbol', 'DZD'),
('currency_name', 'Algerian Dinar'),
('business_name', 'Titaou POS Store'),
('business_address', 'Algiers, Algeria'),
('business_phone', '+213 555 000 000'),
('receipt_paper_width', '80mm'),
('cash_drawer_enabled', '1');
