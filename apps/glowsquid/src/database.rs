use rusqlite_migration::{AsyncMigrations, M};
use tokio_rusqlite::Connection;

#[derive(Clone)]
pub struct DbState {
    connection: Connection,
}

impl DbState {
    /// Initializes and runs migrations on the database
    ///
    /// TODO: error handling and propagation
    /// TODO: logging
    pub async fn new(mut connection: Connection) -> Self {
        let migrations = AsyncMigrations::new(vec![M::up(include_str!(
            "./migrations/0000_init_auth_up.sql"
        ))
        .down(include_str!("./migrations/0000_init_auth_down.sql"))]);

        debug_assert!(
            migrations.validate().await.is_ok(),
            "Migrations are not valid"
        );

        migrations
            .to_latest(&mut connection)
            .await
            .expect("Failed to run migrations");

        // pragma is better to run outside of migrations
        connection
            .call(|conn| Ok(conn.pragma_update(None, "journal_mode", "WAL")))
            .await
            .expect("Failed to send update to database")
            .expect("Failed to set journal mode");

        connection
            .call(|conn| Ok(conn.pragma_update(None, "foreign_keys", "ON")))
            .await
            .expect("Failed to send update to database")
            .expect("Failed to enable foreign keys");

        Self { connection }
    }

    /// Get a connection to the database
    pub fn connection(&self) -> Connection {
        // connection is a cheap clone as it's just a Sender internally
        self.connection.clone()
    }
}
