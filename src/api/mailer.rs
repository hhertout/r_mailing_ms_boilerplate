use actix_web::{post, Result, Responder};
use mailer::Mailer;
use mailer::config::Config;

#[post("/ping")]
pub async fn hello_world() -> Result<impl Responder> {
    Ok("hello")
}

pub fn init_mailer() -> Mailer {
    Mailer { config: Config::new() }
}