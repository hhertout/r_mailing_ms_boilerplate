use std::{io::Result, env};

pub mod api;

#[actix_web::main]
async fn main() -> Result<()>{
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    dotenvy::dotenv().ok();
    api::init().await
}
