use std::io::Error;

use crate::config;
use ::mailer::Mailer;
use actix_web::{
    middleware::Logger,
    web::{Data},
    App, HttpServer,
};
use sqlx::{Pool, Sqlite};
use logs::db::MailerDb;
use crate::router::router;


pub struct AppState {
    pub mailer: Mailer,
    pub db_pool: Pool<Sqlite>,
}


pub async fn init() -> Result<(), Error> {
    let uri = std::env::var("SERVER_URI").unwrap();
    let port = std::env::var("SERVER_PORT").unwrap();

    MailerDb.migrate().await;
    let db_pool = MailerDb::new().database_connection().await;

    println!("📡 Server starting at http://{}:{}/", uri, port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("Request => %s; %a \"%r\" | time => %Dms"))
            .wrap(config::cors::config_cors())
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
