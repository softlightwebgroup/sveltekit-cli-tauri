use std::sync::{Arc, Mutex};

use rusqlite::Connection;

pub struct DatabaseConnection {
    pub conn: Arc<Mutex<Connection>>,
}

impl DatabaseConnection {
    fn add_column(&self, query: Result<usize, rusqlite::Error>, name: &str) {
        match query {
            Ok(_) => println!("Columna '{}' añadida", name),
            Err(e) => {
                println!("{}", e.to_string());
                // Comprueba si el error es porque la columna ya existe
                if e.to_string().contains("duplicate column name") {
                    println!("La columna '{}' ya existe, no se necesita añadir", name);
                } else {
                    // Si el error es por otra razón, lo manejas aquí
                    panic!("Error al añadir la columna '{}': {}", name, e);
                }
            }
        }
    }
    fn create_table(&self) {
        let conn = self.conn.lock().unwrap();

        conn.execute("CREATE TABLE IF NOT EXISTS projects (id uuid PRIMARY KEY, name TEXT, directory TEXT)", [])
            .expect("Failed to create table");
        self.add_column(conn.execute("ALTER TABLE projects ADD COLUMN created_at TIMESTAMP", []), "created_at");
        self.add_column(conn.execute("ALTER TABLE projects ADD COLUMN updated_at TIMESTAMP", []), "updated_at");
        conn.execute("UPDATE projects SET created_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE created_at IS NULL or updated_at IS NULL", [])
            .expect("Failed to update existing rows");
    }
    pub fn new() -> Self {
        let conn = Connection::open("../projects.db").expect("Failed to open database");

        let db = DatabaseConnection {
            conn: Arc::new(Mutex::new(conn))
        };

        db.create_table();

        db
    }
}