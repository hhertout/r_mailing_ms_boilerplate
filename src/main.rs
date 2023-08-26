use std::io::Result;

pub mod api;

#[actix_web::main]
async fn main() -> Result<()>{
    dotenvy::dotenv().ok();
    api::init().await
}
