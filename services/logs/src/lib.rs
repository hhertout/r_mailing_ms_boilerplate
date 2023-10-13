use db::MailerDb;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::{Error, Pool, Sqlite};

pub mod db;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Logs {
    _id: u16,
    method: String,
    to: String,
    date: String,
    success: bool,
}

pub struct LogsRequest {
    pub method: String,
    pub to: String,
    pub date: String,
    pub success: bool,
}

#[derive(Clone)]
pub struct MailerLogs {
    db_pool: Pool<Sqlite>,
}

impl MailerLogs {
    pub async fn new() -> MailerLogs {
        MailerDb.migrate().await;
        let db_pool = MailerDb.database_connection().await;
        MailerLogs { db_pool }
    }

    pub async fn get_logs(&self) -> Result<Vec<Logs>, Error> {
        sqlx::query_as::<_, Logs>(r#"SELECT * FROM `logs`"#)
            .fetch_all(&self.db_pool)
            .await
    }
    pub async fn insert_one(&self, logs: &LogsRequest) -> Result<(), sqlx::Error> {
        let res = sqlx::query(
            "INSERT INTO `logs` (`method`, `to`, `date`, `success`)
            VALUES ($1, $2, $3, $4)",
        )
        .bind(&logs.method)
        .bind(&logs.to)
        .bind(&logs.date)
        .bind(&logs.success)
        .execute(&self.db_pool)
        .await;

        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("{:?}", e);
                Err(e)
            }
        }
    }
}
