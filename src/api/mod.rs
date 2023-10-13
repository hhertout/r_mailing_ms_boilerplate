use std::io::Error;

use crate::config;
use ::mailer::Mailer;
use actix_web::{
    middleware::Logger,
    web::{Data, ServiceConfig},
    App, HttpServer,
};
use logs::MailerLogs;

pub mod mailer;
pub mod mailer_logs;

use crate::api::mailer::*;

use self::mailer_logs::get_mailer_logs;

pub struct AppState {
    pub mailer: Mailer,
    pub mailer_logs: MailerLogs,
}

fn router(cfg: &mut ServiceConfig) {
    cfg.service(hello_world);
    cfg.service(get_mailer_logs);
}

pub async fn init() -> Result<(), Error> {
    let uri = std::env::var("SERVER_URI").unwrap();
    let port = std::env::var("SERVER_PORT").unwrap();

    let mailer_logs = MailerLogs::new().await;

    println!("🚀 Server starting at http://{}:{}/", uri, port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("Request => %s; %a \"%r\" | time => %Dms"))
            .wrap(config::cors::config_cors())
            .app_data(Data::new(AppState {
                mailer: Mailer::new(None),
                mailer_logs: mailer_logs.clone(),
            }))
            .configure(router)
    })
    .bind((uri.as_str(), port.as_str().parse().unwrap()))?
    .run()
    .await
}
