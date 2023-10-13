use std::env;

use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};

pub struct MailerDb;

impl MailerDb {
    pub fn new() -> MailerDb {
        return MailerDb;
    }

    pub async fn database_connection(&self) -> Pool<Sqlite> {
        let db_url = env::var("DB_URL").unwrap();
        SqlitePoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap_or_else(|_| panic!("Failed to connect to database"))
    }

    pub async fn migrate(&self) {
        let _ = &self.create_database().await;
        let pool = &self.database_connection().await;
        let _ = &self.create_logs_table(pool).await;
    }

    async fn create_database(&self) {
        let db_url = env::var("DB_URL").unwrap();
        if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
            match Sqlite::create_database(&db_url).await {
                Ok(_) => println!("Database successfully created"),
                Err(e) => println!("Error during the creation of the database. {}", e),
            };
        }
    }

    async fn create_logs_table(&self, pool: &Pool<Sqlite>) {
        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS `logs` (
                `_id` integer primary key AUTOINCREMENT not null,
                `subject` varchar(255) not null,
                `to` varchar(255) not null, 
                `date` date not null, 
                `success` boolean not null
        );",
        )
        .execute(pool)
        .await
        .unwrap();
    }
}
