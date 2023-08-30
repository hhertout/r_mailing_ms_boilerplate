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
            std::env::var("SMTP_FROM").unwrap_or_else(|_| panic!("Failed to load env variable"));
        let from_name_res = std::env::var("SMTP_FROM_NAME");
        let from_name = match from_name_res {
            Ok(from_name) => Some(from_name),
            Err(_) => None,
        };
        let user = std::env::var("SMTP_USER").expect("STMP_USER must be set");
        let password = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
        let host = std::env::var("SMTP_HOST").expect("SMTP_HOST must be set");
        let port = std::env::var("SMTP_PORT").expect("SMTP_PORT must be set");
        Config {
            from,
            from_name,
            user,
            password,
            host,
            port: port.parse::<u16>().unwrap(),
        }
    }
}
