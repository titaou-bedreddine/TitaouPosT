import os
import sqlite3

appdata = os.environ.get('APPDATA')
db_dir = os.path.join(appdata, 'TitaouPosT')
os.makedirs(db_dir, exist_ok=True)
db_path = os.path.join(db_dir, 'titaou_post.db')

conn = sqlite3.connect(db_path)
cursor = conn.cursor()

# Run 001_initial_schema.sql
with open('src-tauri/migrations/001_initial_schema.sql', 'r', encoding='utf-8') as f:
    cursor.executescript(f.read())

# Run column additions if missing
for col_cmd in [
    'ALTER TABLE categories ADD COLUMN color TEXT DEFAULT "#0284c7"',
    'ALTER TABLE customers ADD COLUMN rc TEXT',
    'ALTER TABLE customers ADD COLUMN nif TEXT',
    'ALTER TABLE customers ADD COLUMN nis TEXT',
    'ALTER TABLE customers ADD COLUMN ai TEXT',
    'ALTER TABLE customers ADD COLUMN qr_code TEXT',
    'ALTER TABLE suppliers ADD COLUMN rc TEXT',
    'ALTER TABLE suppliers ADD COLUMN nif TEXT',
    'ALTER TABLE suppliers ADD COLUMN nis TEXT',
    'ALTER TABLE suppliers ADD COLUMN ai TEXT',
    'ALTER TABLE suppliers ADD COLUMN contact_person TEXT'
]:
    try:
        cursor.execute(col_cmd)
    except Exception as e:
        pass

# Run 002_seed_data.sql
with open('src-tauri/migrations/002_seed_data.sql', 'r', encoding='utf-8') as f:
    cursor.executescript(f.read())

# Ensure default active session
cursor.execute('INSERT OR IGNORE INTO registers (id, name, identifier, is_active) VALUES (1, "Main Register 01", "REG-01", 1)')
cursor.execute('SELECT COUNT(*) FROM cash_sessions WHERE status = "open"')
if cursor.fetchone()[0] == 0:
    cursor.execute('INSERT INTO cash_sessions (id, register_id, user_id, opening_amount, expected_cash, status, notes) VALUES (1, 1, 1, 10000, 10000, "open", "Default Auto-Opened Session")')
    cursor.execute('INSERT INTO cash_movements (session_id, user_id, type, amount, reason) VALUES (1, 1, "opening_balance", 10000, "Startup Cash / رصيد افتتاحي")')

conn.commit()

# Print verification statistics
cursor.execute('SELECT COUNT(*) FROM products')
p_count = cursor.fetchone()[0]
cursor.execute('SELECT COUNT(*) FROM customers')
c_count = cursor.fetchone()[0]
cursor.execute('SELECT COUNT(*) FROM suppliers')
s_count = cursor.fetchone()[0]
cursor.execute('SELECT COUNT(*) FROM users')
u_count = cursor.fetchone()[0]
cursor.execute('SELECT COUNT(*) FROM categories')
cat_count = cursor.fetchone()[0]

conn.close()
print(f'DB Populated Successfully at {db_path}:')
print(f'  - Products: {p_count}')
print(f'  - Customers: {c_count}')
print(f'  - Suppliers: {s_count}')
print(f'  - Users/Employees: {u_count}')
print(f'  - Categories: {cat_count}')