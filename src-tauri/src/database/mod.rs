use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

pub struct DbState {
    pub conn: Mutex<Connection>,
}

impl DbState {
    pub fn new() -> Result<Self> {
        let db_path = get_database_path();
        if let Some(parent) = db_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let conn = Connection::open(&db_path)?;
        
        // Performance PRAGMAs
        conn.execute_batch("
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            PRAGMA foreign_keys = ON;
            PRAGMA busy_timeout = 5000;
        ")?;

        let state = Self {
            conn: Mutex::new(conn),
        };

        state.run_migrations()?;
        state.seed_default_admin()?;
        state.ensure_default_session()?;

        Ok(state)
    }

    pub fn run_migrations(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        
        let m1 = include_str!("../../migrations/001_initial_schema.sql");
        conn.execute_batch(m1)?;

        // Ensure missing columns exist in existing user SQLite databases
        let _ = conn.execute("ALTER TABLE categories ADD COLUMN color TEXT DEFAULT '#0284c7';", []);
        let _ = conn.execute("ALTER TABLE customers ADD COLUMN rc TEXT;", []);
        let _ = conn.execute("ALTER TABLE customers ADD COLUMN nif TEXT;", []);
        let _ = conn.execute("ALTER TABLE customers ADD COLUMN nis TEXT;", []);
        let _ = conn.execute("ALTER TABLE customers ADD COLUMN ai TEXT;", []);
        let _ = conn.execute("ALTER TABLE customers ADD COLUMN qr_code TEXT;", []);
        let _ = conn.execute("ALTER TABLE suppliers ADD COLUMN rc TEXT;", []);
        let _ = conn.execute("ALTER TABLE suppliers ADD COLUMN nif TEXT;", []);
        let _ = conn.execute("ALTER TABLE suppliers ADD COLUMN nis TEXT;", []);
        let _ = conn.execute("ALTER TABLE suppliers ADD COLUMN ai TEXT;", []);
        let _ = conn.execute("ALTER TABLE suppliers ADD COLUMN contact_person TEXT;", []);
        let _ = conn.execute("ALTER TABLE products ADD COLUMN expiry_date TEXT;", []);

        let m2 = include_str!("../../migrations/002_seed_data.sql");
        let _ = conn.execute_batch(m2);

        Ok(())
    }

    fn seed_default_admin(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        
        let _ = conn.execute(
            "INSERT OR IGNORE INTO roles (id, name, description, is_system) VALUES (1, 'Administrator', 'Full system access', 1);",
            [],
        );

        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM users WHERE username = 'admin'",
            [],
            |row| row.get(0),
        ).unwrap_or(0);

        if count == 0 {
            let password = "admin";
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();
            let password_hash = argon2
                .hash_password(password.as_bytes(), &salt)
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))))?
                .to_string();

            conn.execute(
                "INSERT INTO users (username, display_name, password_hash, role_id, max_discount_percent, is_active)
                 VALUES ('admin', 'Administrator', ?1, 1, 100.0, 1)",
                [&password_hash],
            )?;
        }

        Ok(())
    }

    fn ensure_default_session(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        let _ = conn.execute(
            "INSERT OR IGNORE INTO registers (id, name, identifier, is_active) VALUES (1, 'Main Register 01', 'REG-01', 1);",
            [],
        );

        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM cash_sessions WHERE status = 'open'",
            [],
            |r| r.get(0),
        ).unwrap_or(0);

        if count == 0 {
            let _ = conn.execute(
                "INSERT INTO cash_sessions (register_id, user_id, opening_amount, expected_cash, status, notes)
                 VALUES (1, 1, 10000, 10000, 'open', 'Default Auto-Opened Session')",
                [],
            );
            let session_id = conn.last_insert_rowid();
            let _ = conn.execute(
                "INSERT INTO cash_movements (session_id, user_id, type, amount, reason)
                 VALUES (?1, 1, 'opening_balance', 10000, 'Startup Cash / رصيد افتتاحي')",
                [session_id],
            );
        }

        Ok(())
    }
}

pub fn get_database_path() -> PathBuf {
    let mut path = dirs_next().unwrap_or_else(|| PathBuf::from("."));
    path.push("TitaouPosT");
    path.push("titaou_post.db");
    path
}

fn dirs_next() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        std::env::var_os("APPDATA").map(PathBuf::from)
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config"))
    }
}