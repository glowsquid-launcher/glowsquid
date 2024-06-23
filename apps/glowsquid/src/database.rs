use sqlx::{migrate, migrate::MigrateError, SqlitePool};

#[derive(Clone)]
pub struct DbState {
    pool: SqlitePool,
}
impl DbState {
    /// Initializes and runs migrations on the database
    ///
    /// TODO: error handling and propagation
    /// TODO: logging
    pub async fn new(pool: SqlitePool) -> Result<Self, MigrateError> {
        migrate!("db/migrations").run(&pool).await?;

        Ok(Self { pool })
    }

    /// Get a connection to the database
    pub fn connection(&self) -> SqlitePool {
        // connection is a cheap clone as it's just a Sender internally
        self.pool.clone()
    }
}
