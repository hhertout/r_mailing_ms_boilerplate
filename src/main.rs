use std::{io::Result, env};

mod api;
mod config;
mod router;
mod controllers;
mod services;

#[actix_web::main]
async fn main() -> Result<()>{
    env_logger::init();
    dotenvy::dotenv().ok();
    let env_mode = env::var("ENV").unwrap_or_else(|_| panic!("Failed to load global env variable"));
    if  env_mode == "dev" {
        println!("⚠️ Running in dev mode 🔨🔨");
        env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    }
    api::init().await
}
