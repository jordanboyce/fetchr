use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

pub struct Database {
    conn: Mutex<Connection>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub is_folder: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    pub id: String,
    pub collection_id: String,
    pub name: String,
    pub method: String,
    pub url: String,
    pub headers: String, // JSON string
    pub body: String,
    pub body_type: String,
    pub auth_type: String,
    pub auth_data: String, // JSON string
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Environment {
    pub id: String,
    pub name: String,
    pub variables: String, // JSON string
    pub is_active: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct History {
    pub id: String,
    pub method: String,
    pub url: String,
    pub status: i32,
    pub response_time: i32,
    pub created_at: String,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Database {
            conn: Mutex::new(conn),
        };
        db.init_tables()?;
        Ok(db)
    }

    fn init_tables(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS collections (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                parent_id TEXT,
                is_folder INTEGER NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS requests (
                id TEXT PRIMARY KEY,
                collection_id TEXT NOT NULL,
                name TEXT NOT NULL,
                method TEXT NOT NULL,
                url TEXT NOT NULL,
                headers TEXT NOT NULL DEFAULT '[]',
                body TEXT NOT NULL DEFAULT '',
                body_type TEXT NOT NULL DEFAULT 'none',
                auth_type TEXT NOT NULL DEFAULT 'none',
                auth_data TEXT NOT NULL DEFAULT '{}',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (collection_id) REFERENCES collections(id) ON DELETE CASCADE
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS environments (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                variables TEXT NOT NULL DEFAULT '[]',
                is_active INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS history (
                id TEXT PRIMARY KEY,
                method TEXT NOT NULL,
                url TEXT NOT NULL,
                status INTEGER NOT NULL,
                response_time INTEGER NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    // Collections
    pub fn create_collection(&self, collection: &Collection) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO collections (id, name, parent_id, is_folder, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                collection.id,
                collection.name,
                collection.parent_id,
                collection.is_folder as i32,
                collection.created_at
            ],
        )?;
        Ok(())
    }

    pub fn get_all_collections(&self) -> Result<Vec<Collection>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, parent_id, is_folder, created_at FROM collections ORDER BY created_at")?;
        let collections = stmt
            .query_map([], |row| {
                Ok(Collection {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    parent_id: row.get(2)?,
                    is_folder: row.get::<_, i32>(3)? != 0,
                    created_at: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;
        Ok(collections)
    }

    pub fn delete_collection(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM collections WHERE id = ?1", params![id])?;
        Ok(())
    }

    // Requests
    pub fn save_request(&self, request: &Request) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO requests (id, collection_id, name, method, url, headers, body, body_type, auth_type, auth_data, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                request.id,
                request.collection_id,
                request.name,
                request.method,
                request.url,
                request.headers,
                request.body,
                request.body_type,
                request.auth_type,
                request.auth_data,
                request.created_at,
                request.updated_at
            ],
        )?;
        Ok(())
    }

    pub fn get_requests_by_collection(&self, collection_id: &str) -> Result<Vec<Request>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, collection_id, name, method, url, headers, body, body_type, auth_type, auth_data, created_at, updated_at
             FROM requests WHERE collection_id = ?1 ORDER BY created_at"
        )?;
        let requests = stmt
            .query_map(params![collection_id], |row| {
                Ok(Request {
                    id: row.get(0)?,
                    collection_id: row.get(1)?,
                    name: row.get(2)?,
                    method: row.get(3)?,
                    url: row.get(4)?,
                    headers: row.get(5)?,
                    body: row.get(6)?,
                    body_type: row.get(7)?,
                    auth_type: row.get(8)?,
                    auth_data: row.get(9)?,
                    created_at: row.get(10)?,
                    updated_at: row.get(11)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;
        Ok(requests)
    }

    pub fn get_request(&self, id: &str) -> Result<Option<Request>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, collection_id, name, method, url, headers, body, body_type, auth_type, auth_data, created_at, updated_at
             FROM requests WHERE id = ?1"
        )?;
        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(Request {
                id: row.get(0)?,
                collection_id: row.get(1)?,
                name: row.get(2)?,
                method: row.get(3)?,
                url: row.get(4)?,
                headers: row.get(5)?,
                body: row.get(6)?,
                body_type: row.get(7)?,
                auth_type: row.get(8)?,
                auth_data: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn delete_request(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM requests WHERE id = ?1", params![id])?;
        Ok(())
    }

    // Environments
    pub fn save_environment(&self, env: &Environment) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        // If this environment is being set as active, deactivate all others
        if env.is_active {
            conn.execute("UPDATE environments SET is_active = 0", [])?;
        }

        conn.execute(
            "INSERT OR REPLACE INTO environments (id, name, variables, is_active, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                env.id,
                env.name,
                env.variables,
                env.is_active as i32,
                env.created_at
            ],
        )?;
        Ok(())
    }

    pub fn get_all_environments(&self) -> Result<Vec<Environment>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, variables, is_active, created_at FROM environments ORDER BY created_at")?;
        let envs = stmt
            .query_map([], |row| {
                Ok(Environment {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    variables: row.get(2)?,
                    is_active: row.get::<_, i32>(3)? != 0,
                    created_at: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;
        Ok(envs)
    }

    pub fn get_active_environment(&self) -> Result<Option<Environment>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, variables, is_active, created_at FROM environments WHERE is_active = 1")?;
        let mut rows = stmt.query([])?;
        if let Some(row) = rows.next()? {
            Ok(Some(Environment {
                id: row.get(0)?,
                name: row.get(1)?,
                variables: row.get(2)?,
                is_active: row.get::<_, i32>(3)? != 0,
                created_at: row.get(4)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn delete_environment(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM environments WHERE id = ?1", params![id])?;
        Ok(())
    }

    // History
    pub fn add_history(&self, history: &History) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO history (id, method, url, status, response_time, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                history.id,
                history.method,
                history.url,
                history.status,
                history.response_time,
                history.created_at
            ],
        )?;
        Ok(())
    }

    pub fn get_history(&self, limit: i32) -> Result<Vec<History>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, method, url, status, response_time, created_at FROM history ORDER BY created_at DESC LIMIT ?1"
        )?;
        let history = stmt
            .query_map(params![limit], |row| {
                Ok(History {
                    id: row.get(0)?,
                    method: row.get(1)?,
                    url: row.get(2)?,
                    status: row.get(3)?,
                    response_time: row.get(4)?,
                    created_at: row.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;
        Ok(history)
    }

    pub fn clear_history(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM history", [])?;
        Ok(())
    }
}
