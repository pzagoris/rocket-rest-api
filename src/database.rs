use rocket_sync_db_pools::database;

/// DB sqlite pool.
#[database("sqlite_database")]
pub struct DatabasePool(diesel::SqliteConnection);
