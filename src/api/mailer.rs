use actix_web::{post, Result, Responder};


#[post("/ping")]
pub async fn hello_world() -> Result<impl Responder> {
    Ok("hello")
}