use std::io::Error;

use crate::config;
use actix_web::{middleware::Logger, web::{Data}, App, HttpServer};
use sqlx::{Pool, Sqlite};
use crate::middleware::Auth;
use crate::router::router;
use crate::services::logs::db::MailerDb;
use crate::services::mailer::Mailer;

pub struct AppState {
    pub mailer: Mailer,
    pub db_pool: Pool<Sqlite>,
}


pub async fn init() -> Result<(), Error> {
    let uri = std::env::var("SERVER_URI").expect("SERVER_URI env variable is not set");
    let port = std::env::var("SERVER_PORT").expect("SERVER_PORT env variable is not set");

    MailerDb.migrate().await;
    let db_pool = MailerDb::new().database_connection().await;

    println!("📡 Server listening on port {}", port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("Request => %s; %a \"%r\" | time => %Dms"))
            .wrap(config::cors::config_cors())
            .wrap(Auth)
            .app_data(Data::new(AppState {
                mailer: Mailer::new(None),
                db_pool: db_pool.clone(),
            }))
            .configure(router)
    })
        .bind((uri.as_str(), port.as_str().parse().unwrap()))?
        .run()
        .await
}
