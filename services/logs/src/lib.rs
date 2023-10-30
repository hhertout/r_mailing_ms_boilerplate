use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::{Error, Pool, Sqlite};

pub mod db;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Logs {
    _id: u16,
    subject: String,
    to: String,
    date: String,
    success: bool,
    error_desc: Option<String>
}

pub struct LogsRequest {
    pub subject: String,
    pub to: String,
    pub date: String,
    pub success: bool,
    pub error_desc: Option<String>
}

#[derive(Clone)]
pub struct MailerLogs;

impl MailerLogs {
    pub async fn new() -> MailerLogs {
        MailerLogs
    }

    pub async fn get_logs(&self, db_pool: Pool<Sqlite>) -> Result<Vec<Logs>, Error> {
        sqlx::query_as::<_, Logs>(r#"SELECT * FROM `logs`"#)
            .fetch_all(&db_pool)
            .await
    }
    pub async fn insert_one(&self,db_pool: Pool<Sqlite> ,logs: &LogsRequest) -> Result<(), sqlx::Error> {
        let res = sqlx::query(
            "INSERT INTO `logs` (`subject`, `to`, `date`, `success`, `error_desc`)
            VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(&logs.subject)
        .bind(&logs.to)
        .bind(&logs.date)
        .bind(&logs.success)
        .bind(&logs.error_desc)
        .execute(&db_pool)
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
