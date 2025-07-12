//schema.rs

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

// Different types for columns
#[derive(Debug, PartialEq)]
pub enum ColumnType {
    TAG,
    INTEGER,
    FLOAT,
    STRING,
    TIMESTAMP,
}

pub type DbId = u64;
pub type TableId = u64;
pub type ColumnId = u64;

pub struct Database {
    pub id: DbId,
    pub tables: HashMap<String, *mut Table>,
    pub next_table_id: AtomicU64,
}

impl Database {
    pub fn new() -> *mut Self {
        Box::into_raw(Box::new(Database {
            id: 0,
            tables: HashMap::new(),
            next_table_id: AtomicU64::new(1),
        }))
    }

    pub unsafe fn register_table(&mut self, name: String, table: *mut Table) -> Result<TableId, String> {
        if self.tables.contains_key(&name) {
            return Err(format!("Table '{}' already exists", name));
        }

        let id = self.next_table_id.fetch_add(1, Ordering::SeqCst);
        unsafe { (*table).id = id };
        self.tables.insert(name, table);
        Ok(id)
    }

    pub unsafe fn get_table(&self, name: &str) -> Option<*mut Table> {
        self.tables.get(name).copied()
    }

    pub unsafe fn free(mem_addr: *mut Self) {
        unsafe { drop(Box::from_raw(mem_addr)) };
    }
}

pub struct Table {
    pub id: TableId,
    pub columns: HashMap<String, *mut Column>,
    pub id_to_name: HashMap<ColumnId, String>,
    pub next_column_id: AtomicU64,
}

impl Table {
    pub fn new() -> *mut Self {
        Box::into_raw(Box::new(Table {
            id: 0,
            columns: HashMap::new(),
            id_to_name: HashMap::new(),
            next_column_id: AtomicU64::new(1),
        }))
    }

    pub unsafe fn register_column(&mut self, name: String, column: *mut Column) -> Result<ColumnId, String> {
        if self.columns.contains_key(&name) {
            return Err(format!("Column '{}' already exists", name));
        }

        let id = self.next_column_id.fetch_add(1, Ordering::SeqCst);
        unsafe { (*column).id = id };
        self.columns.insert(name.clone(), column);
        self.id_to_name.insert(id, name);
        Ok(id)
    }

    pub unsafe fn get_column(&self, name: &str) -> Option<*mut Column> {
        self.columns.get(name).copied()
    }

    pub unsafe fn free(mem_addr: *mut Self) {
        unsafe { drop(Box::from_raw(mem_addr)) };
    }
}

pub struct Column {
    pub id: ColumnId,
    pub r#type: ColumnType,
}

impl Column {
    pub fn new(column_type: ColumnType) -> *mut Self {
        Box::into_raw(Box::new(Column {
            id: 0,
            r#type: column_type,
        }))
    }
}

