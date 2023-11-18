use std::env;

use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};

#[derive(Default)]
pub struct MailerDb;

impl MailerDb {
    pub fn new() -> MailerDb {
        MailerDb
    }

    pub async fn database_connection(&self) -> Pool<Sqlite> {
        let db_url = env::var("DB_URL").unwrap();
        SqlitePoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap_or_else(|_| panic!("Failed to connect to database. {}", db_url))
    }

    pub async fn migrate(&self) {
        let _ = &self.create_database().await;
        let pool = &self.database_connection().await;
        let _ = &self.migrations_migrate(pool).await;
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

    async fn migrations_migrate(&self, pool: &Pool<Sqlite>) {
        sqlx::migrate!()
            .run(pool)
            .await
            .unwrap_or_else(|err| panic!("Migration failed : {:?}", err))
    }
}
