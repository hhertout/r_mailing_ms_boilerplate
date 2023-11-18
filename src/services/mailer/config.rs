#[derive(Default)]
pub struct Config {
    pub from: String,
    pub from_name: Option<String>,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

impl Config {
    pub fn new() -> Config {
        let from =
            std::env::var("SMTP_FROM").expect("SMTP_FROM env variable is not set");
        let from_name_res = std::env::var("SMTP_FROM_NAME");
        let from_name = match from_name_res {
            Ok(from_name) => Some(from_name),
            Err(_) => None,
        };
        let user = std::env::var("SMTP_USER").expect("SMTP_USER env variable is not set");
        let password = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD env variable is not set");
        let host = std::env::var("SMTP_HOST").expect("SMTP_HOST env variable is not set");
        let port = std::env::var("SMTP_PORT").expect("SMTP_PORT env variable is not set");
        Config {
            from,
            from_name,
            user,
            password,
            host,
            port: port.parse::<u16>().expect("Failed to parse port"),
        }
    }
}
