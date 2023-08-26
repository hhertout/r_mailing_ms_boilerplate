use std::io::Error;

use actix_web::{middleware::Logger, App, HttpServer, web::ServiceConfig};

pub mod mailer;

use crate::api::mailer::*;

fn router(cfg: &mut ServiceConfig) {
    cfg.service(hello_world);
}

pub async fn init() -> Result<(), Error> {
    let uri = std::env::var("SERVER_URI").unwrap_or_else(|_| "localhost".to_string());
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "4000".to_string());

    HttpServer::new(move || {
        App::new().wrap(Logger::new(
            "Request => %a \"%r\"; status => %s; time => %Dms",
        ))
        .configure(router)
    })
    .bind((uri.as_str(), port.as_str().parse().unwrap()))?
    .run()
    .await
}
