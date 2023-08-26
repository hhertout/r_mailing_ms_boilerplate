use actix_web::{get, Result, Responder, web};
use mailer::Mailer;
use mailer::config::Config;

#[get("/ping")]
pub async fn hello_world() -> Result<impl Responder> {
    Ok(web::Json("Hello"))
}

pub fn init_mailer() -> Mailer {
    Mailer { config: Config::new() }
}