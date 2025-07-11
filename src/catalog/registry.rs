use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use super::schema::{Database, Table};

// Catalog keeps metadata of multiple Databases
// Each host can hold a single Catalog for now
pub struct Catalog {
    pub databases: HashMap<String, *mut Database>,
    pub next_db_id: AtomicU64,
}

impl Catalog {
    pub fn new() -> *mut Self {
        Box::into_raw(Box::new(Catalog {
            databases: HashMap::new(),
            next_db_id: AtomicU64::new(0),
        }))
    }

    // Later change the String param inside the Result to i32 with enum errors
    pub unsafe fn register_database(&mut self, name: String, db: *mut Database) -> Result<u64, String> {
        if self.databases.contains_key(&name) {
            return Err(format!("Database '{}' already exists", name));
        }

        let id = self.next_db_id.fetch_add(1, Ordering::SeqCst);
        unsafe { (*db).id = id };
        self.databases.insert(name, db);
        Ok(id)
    }

    pub unsafe fn get_database(&self, name: &str) -> Option<*mut Database> {
        self.databases.get(name).copied()
    }

    pub unsafe fn free(mem_addr: *mut Self) {
        unsafe { drop(Box::from_raw(mem_addr)) };
    }
}

