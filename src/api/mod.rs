use std::io::Error;

use actix_web::{middleware::Logger, App, HttpServer, web::{ServiceConfig, Data}};

pub mod mailer;

use crate::api::mailer::*;

fn router(cfg: &mut ServiceConfig) {
    cfg.service(hello_world);
}

pub async fn init() -> Result<(), Error> {
    let uri = std::env::var("SERVER_URI").unwrap();
    let port = std::env::var("SERVER_PORT").unwrap();

    println!("🚀 Server currently running at http://{}:{}/", uri, port);
    HttpServer::new(move || {
        App::new().wrap(Logger::new(
            "Request => %a \"%r\"; status => %s; time => %Dms",
        ))
        .app_data(Data::new(init_mailer()))
        .configure(router)
    })
    .bind((uri.as_str(), port.as_str().parse().unwrap()))?
    .run()
    .await
}
