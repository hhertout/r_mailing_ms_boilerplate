use std::io::Error;

use ::mailer::Mailer;
use actix_web::{
    middleware::Logger,
    web::{Data, ServiceConfig},
    App, HttpServer,
};

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

    println!("🚀 Server currently running at http://{}:{}/", uri, port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new(
                "Request => %s; %a \"%r\" | time => %Dms",
            ))
            .app_data(Data::new(AppState {
                mailer: Mailer::new(),
            }))
            .configure(router)
    })
    .bind((uri.as_str(), port.as_str().parse().unwrap()))?
    .run()
    .await
}
