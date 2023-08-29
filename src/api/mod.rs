use std::io::Error;

use ::mailer::Mailer;
use actix_web::{
    middleware::Logger,
    web::{Data, ServiceConfig},
    App, HttpServer,
};
use crate::config;

pub mod mailer;

use crate::api::mailer::*;

pub struct AppState {
    pub mailer: Mailer,
}

fn router(cfg: &mut ServiceConfig) {
    cfg.service(hello_world);
}

pub async fn init() -> Result<(), Error> {
    let uri = std::env::var("SERVER_URI").unwrap();
    let port = std::env::var("SERVER_PORT").unwrap();

    println!("🚀 Server starting at http://{}:{}/", uri, port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new(
                "Request => %s; %a \"%r\" | time => %Dms",
            ))
            .wrap(config::cors::config_cors())
            .app_data(Data::new(AppState {
                mailer: Mailer::new(None),
            }))
            .configure(router)
    })
    .bind((uri.as_str(), port.as_str().parse().unwrap()))?
    .run()
    .await
}
