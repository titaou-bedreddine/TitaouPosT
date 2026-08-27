import random

categories = [
    (1, "المواد الغذائية العامة", "Alimentation Générale", "General Groceries", "#0284c7"),
    (2, "المشروبات والعصائر", "Boissons & Jus", "Beverages & Juices", "#0ea5e9"),
    (3, "منتجات الحليب والألبان", "Produits Laitiers", "Dairy Products", "#38bdf8"),
    (4, "البسكويت والحلويات", "Biscuiterie & Chocolat", "Snacks & Sweets", "#f59e0b"),
    (5, "الزيوت والمعلبات", "Huiles & Conserves", "Oils & Canned Goods", "#10b981"),
    (6, "مواد التنظيف والغسيل", "Produits d''Entretien", "Cleaning Products", "#6366f1"),
    (7, "العناية الشخصية والتجميل", "Hygiène & Beauté", "Personal Care & Beauty", "#ec4899"),
    (8, "البقوليات والتوابل", "Épices & Féculents", "Spices & Grains", "#84cc16"),
    (9, "المخبوزات والفرينة", "Boulangerie & Farines", "Bakery & Flours", "#d97706"),
    (10, "الإلكترونيات والملحقات", "Électronique & Accessoires", "Electronics & Accessories", "#8b5cf6")
]

products_templates = [
    (1, "سكر أبيض سيفيتال 1 كغ", "Sucre Blanc Cevital 1kg", "Cevital White Sugar 1kg", 85, 100),
    (1, "فرينة سيم فاخرة 1 كغ", "Farine Sim Extra 1kg", "Sim Extra Flour 1kg", 50, 65),
    (1, "قهوة فاميكو 250 غ", "Café Famico Moulu 250g", "Famico Ground Coffee 250g", 240, 290),
    (1, "شاي أحمر المنيعة 250 غ", "Thé Rouge El Meniaa 250g", "El Meniaa Red Tea 250g", 180, 230),
    (1, "كسكسي ماما متوسط 1 كغ", "Couscous Mama Moyen 1kg", "Mama Medium Couscous 1kg", 130, 160),
    (1, "معكرونة سيم سباغيتي 500 غ", "Spaghetti Sim 500g", "Sim Spaghetti 500g", 70, 90),
    (1, "أرز بسمتي الشعلان 1 كغ", "Riz Basmati Al Shalan 1kg", "Al Shalan Basmati Rice 1kg", 320, 390),
    (2, "حمود بوعلام رامي 1 لتر", "Hamoud Boualem Selecto 1L", "Hamoud Boualem Selecto 1L", 90, 110),
    (2, "ماء معدني إفرو 1.5 لتر", "Eau Minérale Ifri 1.5L", "Ifri Mineral Water 1.5L", 35, 45),
    (2, "عصير رامي برتقال 1 لتر", "Jus Ramy Orange 1L", "Ramy Orange Juice 1L", 120, 150),
    (2, "كوكاكولا قارورة 1 لتر", "Coca-Cola Bouteille 1L", "Coca-Cola Bottle 1L", 115, 140),
    (2, "عصير نغاووس مشمش 1 لتر", "Jus N''Gaous Abricot 1L", "N''Gaous Apricot Juice 1L", 130, 165),
    (3, "حليب كانديا نصف دسم 1 لتر", "Lait Candia Silhouette 1L", "Candia Silhouette Milk 1L", 125, 150),
    (3, "جبن بريزيدون مثلثات 16 قطعة", "Fromage Président 16 Portions", "President Cheese 16 Portions", 260, 320),
    (3, "ياغورت صومام فواكه 100 غ", "Yaourt Soummam Fruits 100g", "Soummam Fruit Yogurt 100g", 30, 40),
    (3, "زبدة بريزيدون غير مملحة 200 غ", "Beurre Président Doux 200g", "President Butter 200g", 380, 460),
    (3, "جبن مبشور فريكو غودا 200 غ", "Fromage Râpé Gouda Frico 200g", "Frico Grated Gouda 200g", 420, 520),
    (4, "بسكويت بيمو ماكاو 150 غ", "Biscuit Bimo Macao 150g", "Bimo Macao Biscuits 150g", 65, 85),
    (4, "شوكولاتة موناكو بالبندق 100 غ", "Chocolat Monaco Noisettes 100g", "Monaco Hazelnut Chocolate 100g", 140, 180),
    (4, "قوفريط ماكسي شوكولا 200 غ", "Gaufrettes Maxi Choco 200g", "Maxi Choco Wafers 200g", 90, 120),
    (4, "حلوى كابريس بالكراميل 250 غ", "Bonbons Caprice Caramel 250g", "Caprice Caramel Candies 250g", 170, 220),
    (5, "زيت طهي إيليو سيفيتال 5 لتر", "Huile de Table Elio 5L", "Elio Cooking Oil 5L", 620, 750),
    (5, "طماطم مصبرة كاب 800 غ", "Concentré Tomate CAB 800g", "CAB Tomato Paste 800g", 210, 260),
    (5, "تونة بالزيت النباتي ماريور 160 غ", "Thon MarYor à l''Huile 160g", "MarYor Tuna in Oil 160g", 190, 240),
    (5, "مايونيز ديورا 500 مل", "Mayonnaise Diura 500ml", "Diura Mayonnaise 500ml", 220, 280),
    (6, "سائل غسيل الأواني إيزيس 1 لتر", "Liquide Vaisselle ISIS 1L", "ISIS Dishwashing Liquid 1L", 160, 200),
    (6, "مسحوق غسيل أومو أوتوماتيك 3 كغ", "Lessive OMO Automatique 3kg", "OMO Auto Laundry Powder 3kg", 850, 1050),
    (6, "جافيل جودور 2 لتر", "Eau de Javel Javelot 2L", "Javelot Bleach 2L", 110, 140),
    (7, "شامبو فينوس زيت الأرغان 400 مل", "Shampoing Vénus Huile d''Argan 400ml", "Venus Argan Shampoo 400ml", 240, 310),
    (7, "صابون دوف مرطب 100 غ", "Savon Dove Hydratant 100g", "Dove Moisturizing Soap 100g", 130, 170),
    (7, "معجون أسنان سيجنال مكافحة التسوس", "Dentifrice Signal Anti-Caries 75ml", "Signal Toothpaste 75ml", 150, 195),
    (8, "حمص حب سفينة 1 كغ", "Pois Chiches Safina 1kg", "Safina Chickpeas 1kg", 280, 350),
    (8, "عدس بني ممتاز 1 كغ", "Lentilles Brunes Extra 1kg", "Extra Brown Lentils 1kg", 240, 300),
    (9, "خميرة الخبز ساف إنستنت 500 غ", "Levure Boulangère Saf-Instant 500g", "Saf-Instant Yeast 500g", 310, 390),
    (10, "كابل شحن سريع تايب سي 1 متر", "Câble USB-C Fast Charge 1m", "Fast Charge USB-C Cable 1m", 250, 450),
    (10, "سماعات أذن سلكية ستيريو", "Écouteurs Filaire Stéréo Jack 3.5mm", "Stereo In-Ear Earphones", 300, 550)
]

sql_statements = []

# 0. Foundation Roles, Units, Registers
sql_statements.append("-- Seed Roles")
sql_statements.append("INSERT OR IGNORE INTO roles (id, name, description, is_system) VALUES (1, 'Administrator', 'Full system access', 1);")
sql_statements.append("INSERT OR IGNORE INTO roles (id, name, description, is_system) VALUES (2, 'Cashier', 'POS and checkout access', 1);")
sql_statements.append("INSERT OR IGNORE INTO roles (id, name, description, is_system) VALUES (3, 'Manager', 'Store operations and reports', 1);")
sql_statements.append("INSERT OR IGNORE INTO roles (id, name, description, is_system) VALUES (4, 'Inventory Clerk', 'Stock and product management', 1);")

sql_statements.append("\n-- Seed Units")
sql_statements.append("INSERT OR IGNORE INTO units (id, name, short_name, allow_decimals) VALUES (1, 'Piece / Pièce / قطعة', 'pcs', 0);")
sql_statements.append("INSERT OR IGNORE INTO units (id, name, short_name, allow_decimals) VALUES (2, 'Kilogram / Kilogramme / كيلوغرام', 'kg', 1);")
sql_statements.append("INSERT OR IGNORE INTO units (id, name, short_name, allow_decimals) VALUES (3, 'Liter / Litre / لتر', 'L', 1);")
sql_statements.append("INSERT OR IGNORE INTO units (id, name, short_name, allow_decimals) VALUES (4, 'Pack / Paquet / علبة', 'pck', 0);")
sql_statements.append("INSERT OR IGNORE INTO units (id, name, short_name, allow_decimals) VALUES (5, 'Box / Carton / كرتون', 'box', 0);")

sql_statements.append("\n-- Seed Registers")
sql_statements.append("INSERT OR IGNORE INTO registers (id, name, identifier, is_active) VALUES (1, 'Main Register 01', 'REG-01', 1);")
sql_statements.append("INSERT OR IGNORE INTO registers (id, name, identifier, is_active) VALUES (2, 'Secondary Register 02', 'REG-02', 1);")

# 1. Categories
sql_statements.append("\n-- Seed Categories")
sql_statements.append("DELETE FROM categories;")
for cid, ar, fr, en, col in categories:
    sql_statements.append(f"INSERT INTO categories (id, name_ar, name_fr, name_en, color, is_active) VALUES ({cid}, '{ar}', '{fr}', '{en}', '{col}', 1);")

# 2. Products & Barcodes (1000 items)
sql_statements.append("\n-- Seed 1000 Products & Barcodes")
sql_statements.append("DELETE FROM product_barcodes;")
sql_statements.append("DELETE FROM products;")

random.seed(42)

for i in range(1, 1001):
    tmpl = products_templates[(i - 1) % len(products_templates)]
    cid = tmpl[0]
    var_idx = (i // len(products_templates)) + 1
    
    sku = f"PRD-{i:04d}"
    ar_name = f"{tmpl[1]} (حجم {var_idx})" if var_idx > 1 else tmpl[1]
    fr_name = f"{tmpl[2]} (Vol {var_idx})" if var_idx > 1 else tmpl[2]
    en_name = f"{tmpl[3]} (Size {var_idx})" if var_idx > 1 else tmpl[3]
    
    price_factor = 1.0 + ((var_idx - 1) * 0.15)
    purchase = int(tmpl[4] * price_factor)
    sale = int(tmpl[5] * price_factor)
    min_sale = int(purchase * 1.05)
    stock = random.randint(15, 250)
    
    sql_statements.append(
        f"INSERT INTO products (id, sku, name_ar, name_fr, name_en, category_id, unit_id, purchase_price, sale_price, min_sale_price, tax_rate, current_stock, min_stock, is_active) "
        f"VALUES ({i}, '{sku}', '{ar_name}', '{fr_name}', '{en_name}', {cid}, 1, {purchase}, {sale}, {min_sale}, 19, {stock}, 5, 1);"
    )
    
    barcode_ean = f"613{i:09d}"
    barcode_alt = f"ALT{i:05d}"
    sql_statements.append(f"INSERT INTO product_barcodes (product_id, barcode, is_primary) VALUES ({i}, '{barcode_ean}', 1);")
    sql_statements.append(f"INSERT INTO product_barcodes (product_id, barcode, is_primary) VALUES ({i}, '{barcode_alt}', 0);")

# 3. Customers (100 customers)
sql_statements.append("\n-- Seed 100 Customers")
sql_statements.append("DELETE FROM customer_debt_payments;")
sql_statements.append("DELETE FROM customers;")

wilayas = ["Alger", "Oran", "Constantine", "Setif", "Blida", "Annaba", "Batna", "Tlemcen", "Bejaia", "Tizi Ouzou", "Chlef", "Biskra", "Mostaganem", "Boumerdes"]
first_names = ["Mohamed", "Karim", "Yacine", "Ahmed", "Samir", "Amine", "Sofiane", "Walid", "Nabil", "Fouad", "Rachid", "Mustapha", "Khaled", "Hocine", "Ali"]
last_names = ["Benali", "Mansouri", "Brahimi", "Cherif", "Bouzid", "Khelifi", "Belkacem", "Saidi", "Hamidi", "Zerrouki", "Meziane", "Dahmani", "Guerfi", "Taleb"]
companies = ["Supermarché", "Supérette", "Épicerie", "Cafétéria", "Restaurant", "Alimentation Générale", "Boulangerie", "Hôtel"]

for c in range(1, 101):
    wilaya = wilayas[c % len(wilayas)]
    if c % 3 == 0:
        name = f"{companies[c % len(companies)]} {last_names[c % len(last_names)]}"
    else:
        name = f"{first_names[c % len(first_names)]} {last_names[c % len(last_names)]}"
    
    phone = f"055{c:07d}" if c % 2 == 0 else f"066{c:07d}"
    debt = (c * 750) % 45000 if c % 4 != 0 else 0
    qr = f"CUST-QR-{c:03d}"
    rc = f"16/00-{c:07d}B22"
    nif = f"0016{c:011d}"
    nis = f"198016{c:07d}"
    ai = f"16010{c:06d}"
    
    sql_statements.append(
        f"INSERT INTO customers (id, name, phone, email, address, rc, nif, nis, ai, qr_code, balance, initial_debt, is_active) "
        f"VALUES ({c}, '{name}', '{phone}', 'client{c}@mail.dz', '{wilaya}, Algérie', '{rc}', '{nif}', '{nis}', '{ai}', '{qr}', {debt}, {debt}, 1);"
    )

# 4. Suppliers (20 suppliers)
sql_statements.append("\n-- Seed 20 Suppliers")
sql_statements.append("DELETE FROM supplier_debt_payments;")
sql_statements.append("DELETE FROM purchase_items;")
sql_statements.append("DELETE FROM purchases;")
sql_statements.append("DELETE FROM suppliers;")

suppliers_list = [
    ("Groupe Cevital Agro-Industrie", "034211122", "contact@cevital.com", "Béjaïa, Algérie", "Issad Rebrab", 150000),
    ("Groupe Danone Djurdjura Algérie", "026123456", "danone@danone.dz", "Akbou, Béjaïa", "Karim Djurdjura", 85000),
    ("Ifri Boissons & Eau Minérale", "034334455", "commercial@ifri.dz", "Ighzer Amokrane, Béjaïa", "Lounis Ibrahim", 45000),
    ("Candia Tchin-Lait Algérie", "034198765", "tchinlait@candia.dz", "Béjaïa Zone Industrielle", "Fawzi Berkane", 120000),
    ("Bimo Biscuiterie Industrielle", "024887766", "bimo@bimo.dz", "Baba Ali, Alger", "Mourad Bimo", 30000),
    ("Henkel Algérie Détergents", "023998877", "henkel@henkel.dz", "Rouïba Zone Industrielle, Alger", "Sofiane Henkel", 65000),
    ("Palmary Food Industries", "025443322", "palmary@palmary.dz", "Boufarik, Blida", "Tarek Palmary", 40000),
    ("Groupe Amor Benamor Céréales", "037889900", "benamor@benamor.dz", "Guelma, Algérie", "Laid Benamor", 95000),
    ("Laboratoires Vénus Saphir", "023112233", "venus@venus.dz", "Oued Smar, Alger", "Samia Venus", 25000),
    ("Condor Electronics SPA", "035667788", "condor@condor.dz", "Bordj Bou Arreridj", "Abderrahmane Benhamadi", 200000),
    ("Laiterie Soummam", "034255566", "soummam@soummam.dz", "Akbou, Béjaïa", "Lounis Hamitouche", 110000),
    ("Ramy Boissons & Jus", "023776655", "ramy@ramy.dz", "Zone Industrielle Oued Smar, Alger", "Ali Ramy", 55000),
    ("Moulins Industriels Safina", "036998811", "safina@safina.dz", "Sétif, Algérie", "Hocine Safina", 75000),
    ("Hamoud Boualem SPA", "021665544", "hamoud@hamoud.dz", "Hassiba Ben Bouali, Alger", "Yacine Hamoud", 60000),
    ("N''Gaous Conserves & Boissons", "033887766", "ngaous@ngaous.dz", "N''Gaous, Batna", "Messaoud Ngaous", 35000),
    ("Faderco Hygiène & Papier", "023554433", "faderco@faderco.dz", "Setif & Alger", "Amor Habes", 80000),
    ("Générale des Huiles Végétales", "041223344", "ghv@ghv.dz", "Oran Zone Portuaire", "Kamel Dahmani", 130000),
    ("Sim Agro-Alimentaire", "025778899", "sim@groupesim.com", "Ain Defla, Algérie", "Abdelkader Tayeb", 90000),
    ("Mami Plast & Emballages", "024556677", "mami@mamiplast.dz", "Boumerdès, Algérie", "Rachid Mami", 20000),
    ("Extra Distribution Matériel", "021990011", "contact@extradistrib.dz", "Hydra, Alger", "Farid Extra", 45000)
]

for s, (sname, sphone, semail, saddr, sperson, sdebt) in enumerate(suppliers_list, 1):
    src = f"16/00-{s:07d}A20"
    snif = f"0020{s:011d}"
    snis = f"197516{s:07d}"
    sai = f"16020{s:06d}"
    sql_statements.append(
        f"INSERT INTO suppliers (id, name, phone, email, address, rc, nif, nis, ai, contact_person, balance, is_active) "
        f"VALUES ({s}, '{sname}', '{sphone}', '{semail}', '{saddr}', '{src}', '{snif}', '{snis}', '{sai}', '{sperson}', {sdebt}, 1);"
    )

# 5. Employees & Users
sql_statements.append("\n-- Seed Employees & Users")
p_hash = "$argon2id$v=19$m=19456,t=2,p=1$ZGVmYXVsdHNhbHQxMjM0NQ$r8nZ9LhUqfUfH9C/N51j+V3QoQ1k6Lp4vX9rN7y8b4Q"

sql_statements.append(
    f"INSERT OR REPLACE INTO users (id, username, display_name, password_hash, pin_hash, role_id, max_discount_percent, is_active) "
    f"VALUES (1, 'admin', 'Administrator', '{p_hash}', '1234', 1, 100.0, 1);"
)
sql_statements.append(
    f"INSERT OR REPLACE INTO users (id, username, display_name, password_hash, pin_hash, role_id, max_discount_percent, is_active) "
    f"VALUES (2, 'kamel', 'Kamel Zerrouki', '{p_hash}', '1111', 2, 10.0, 1);"
)
sql_statements.append(
    f"INSERT OR REPLACE INTO users (id, username, display_name, password_hash, pin_hash, role_id, max_discount_percent, is_active) "
    f"VALUES (3, 'amina', 'Amina Cherif', '{p_hash}', '2222', 2, 15.0, 1);"
)
sql_statements.append(
    f"INSERT OR REPLACE INTO users (id, username, display_name, password_hash, pin_hash, role_id, max_discount_percent, is_active) "
    f"VALUES (4, 'samir', 'Samir Bouzid', '{p_hash}', '9999', 3, 30.0, 1);"
)

with open("src-tauri/migrations/002_seed_data.sql", "w", encoding="utf-8") as f:
    f.write("\n".join(sql_statements))

print("Successfully regenerated 002_seed_data.sql!")