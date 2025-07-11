pub mod catalog;
pub mod engine;
pub mod net;
pub mod types;
pub mod utils;
pub mod config;

// public exports from the `catalog` module
pub use catalog::{Catalog, Database, Table, Column, ColumnType};

#[cfg(test)]
mod tests {
    use super::catalog::*;

    #[test]
    fn time_series_setup() {
        unsafe {
            let catalog = Catalog::new();
            let db = Database::new();

            assert!( (*catalog)
                .register_database("monitoring".to_string(), db)
                .is_ok() );

            let db_ptr = (*catalog).get_database("monitoring").unwrap();
            let table = Table::new();

            assert!( (*db_ptr)
                .register_table("cpu_usage".to_string(), table)
                .is_ok() );

            let table_ptr = (*db_ptr).get_table("cpu_usage").unwrap();

            let ts_col = Column::new(ColumnType::TIMESTAMP);
            let value_col = Column::new(ColumnType::FLOAT);
            let host_col = Column::new(ColumnType::TAG);

            assert!( (*table_ptr)
                .register_column("timestamp".to_string(), ts_col)
                .is_ok() );
            assert!( (*table_ptr)
                .register_column("value".to_string(), value_col)
                .is_ok() );
            assert!( (*table_ptr)
                .register_column("host".to_string(), host_col)
                .is_ok() );

            let timestamp_col_ptr = (*table_ptr).get_column("timestamp").unwrap();
            let value_col_ptr = (*table_ptr).get_column("value").unwrap();
            let host_col_ptr = (*table_ptr).get_column("host").unwrap();

            assert_eq!((*timestamp_col_ptr).id, 1);
            assert_eq!((*value_col_ptr).id, 2);
            assert_eq!((*host_col_ptr).id, 3);

            drop(Box::from_raw(catalog));
            drop(Box::from_raw(db));
            drop(Box::from_raw(table));
        }
    }
}

