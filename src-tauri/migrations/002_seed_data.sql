-- TitaouPOS Core Starter Seed (Clean Production Ready Database)

-- 1. Seed Roles
INSERT OR IGNORE INTO roles (id, name, description, is_system) VALUES (1, 'Administrator', 'Full system access with all administrative privileges', 1);
INSERT OR IGNORE INTO roles (id, name, description, is_system) VALUES (2, 'Cashier', 'POS and checkout register access', 1);
INSERT OR IGNORE INTO roles (id, name, description, is_system) VALUES (3, 'Manager', 'Store operations, inventory, and reporting', 1);
INSERT OR IGNORE INTO roles (id, name, description, is_system) VALUES (4, 'Inventory Clerk', 'Stock intake and product catalog management', 1);

-- 2. Seed Standard Units
INSERT OR IGNORE INTO units (id, name, short_name, allow_decimals) VALUES (1, 'Piece / Pièce / قطعة', 'pcs', 0);
INSERT OR IGNORE INTO units (id, name, short_name, allow_decimals) VALUES (2, 'Kilogram / Kilogramme / كيلوغرام', 'kg', 1);
INSERT OR IGNORE INTO units (id, name, short_name, allow_decimals) VALUES (3, 'Liter / Litre / لتر', 'L', 1);
INSERT OR IGNORE INTO units (id, name, short_name, allow_decimals) VALUES (4, 'Pack / Paquet / علبة', 'pck', 0);
INSERT OR IGNORE INTO units (id, name, short_name, allow_decimals) VALUES (5, 'Box / Carton / كرتون', 'box', 0);

-- 3. Seed Registers
INSERT OR IGNORE INTO registers (id, name, identifier, is_active) VALUES (1, 'Main Register 01', 'REG-01', 1);
INSERT OR IGNORE INTO registers (id, name, identifier, is_active) VALUES (2, 'Secondary Register 02', 'REG-02', 1);

-- 4. Seed Clean Starter Categories
INSERT OR IGNORE INTO categories (id, name_ar, name_fr, name_en, color, is_active) VALUES (1, 'افتراضي (Default)', 'Général / Default', 'Default', '#0284c7', 1);
INSERT OR IGNORE INTO categories (id, name_ar, name_fr, name_en, color, is_active) VALUES (2, 'المشروبات والعصائر', 'Boissons & Jus', 'Beverages & Juices', '#0ea5e9', 1);
INSERT OR IGNORE INTO categories (id, name_ar, name_fr, name_en, color, is_active) VALUES (3, 'منتجات الحليب والألبان', 'Produits Laitiers', 'Dairy Products', '#38bdf8', 1);
INSERT OR IGNORE INTO categories (id, name_ar, name_fr, name_en, color, is_active) VALUES (4, 'البسكويت والحلويات', 'Biscuiterie & Chocolat', 'Snacks & Sweets', '#f59e0b', 1);
INSERT OR IGNORE INTO categories (id, name_ar, name_fr, name_en, color, is_active) VALUES (5, 'الزيوت والمعلبات', 'Huiles & Conserves', 'Oils & Canned Goods', '#10b981', 1);
INSERT OR IGNORE INTO categories (id, name_ar, name_fr, name_en, color, is_active) VALUES (6, 'مواد التنظيف والغسيل', 'Produits d''Entretien', 'Cleaning Products', '#6366f1', 1);
INSERT OR IGNORE INTO categories (id, name_ar, name_fr, name_en, color, is_active) VALUES (7, 'العناية الشخصية والتجميل', 'Hygiène & Beauté', 'Personal Care & Beauty', '#ec4899', 1);
INSERT OR IGNORE INTO categories (id, name_ar, name_fr, name_en, color, is_active) VALUES (8, 'البقوليات والتوابل', 'Épices & Féculents', 'Spices & Grains', '#84cc16', 1);
INSERT OR IGNORE INTO categories (id, name_ar, name_fr, name_en, color, is_active) VALUES (9, 'المخبوزات والفرينة', 'Boulangerie & Farines', 'Bakery & Flours', '#d97706', 1);
INSERT OR IGNORE INTO categories (id, name_ar, name_fr, name_en, color, is_active) VALUES (10, 'الإلكترونيات والملحقات', 'Électronique & Accessoires', 'Electronics & Accessories', '#8b5cf6', 1);

-- 5. Seed Default Walk-in Customer
INSERT OR IGNORE INTO customers (id, name, code, phone, qr_code, balance, is_active)
VALUES (1, 'Client Comptoir / زبون عادي', 'CUST-001', '0550000000', 'CUST-001', 0, 1);