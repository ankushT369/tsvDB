//mod.rs

pub mod registry;
pub mod schema;
pub mod chunk_meta;
pub mod loader;

pub use registry::Catalog;
pub use schema::{Database, Table, Column, ColumnType};

