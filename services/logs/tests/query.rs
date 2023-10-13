use std::env;

use logs::{LogsRequest, MailerLogs};

#[tokio::test]
pub async fn query_all_test() {
    env::set_var("DB_URL", "sqlite://../../data/logs_test.db");
    let res = MailerLogs::new().await.get_logs().await;
    assert!(res.is_ok())
}

#[tokio::test]
pub async fn insert_one_test() {
    env::set_var("DB_URL", "sqlite://../../data/logs_test.db");
    let logs = LogsRequest {
        subject: String::from("test"),
        to: String::from("test@test.com"),
        date: String::from("2023-10-13"),
        success: true,
        error_desc: None
    };
    let res = MailerLogs::new().await.insert_one(&logs).await;
    assert!(res.is_ok());

    let logs = LogsRequest {
        subject: String::from("test"),
        to: String::from("test@test.com"),
        date: String::from("2023-10-13"),
        success: true,
        error_desc: Some("hello".to_owned())
    };
    let res = MailerLogs::new().await.insert_one(&logs).await;
    assert!(res.is_ok());
}
