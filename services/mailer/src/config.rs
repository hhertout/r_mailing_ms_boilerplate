pub struct Config {
    pub from: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

impl Config {
    pub fn new() -> Config {
        let from =
            std::env::var("SMTP_FROM").unwrap_or_else(|_| panic!("Failed to load env variable"));
        let user = std::env::var("SMTP_USER").expect("STMP_USER must be set");
        let password = std::env::var("STMP_PASSWORD").expect("SMTP_PASSWORD must be set");
        let host = std::env::var("SMTP_HOST").expect("SMTP_HOST must be set");
        let port = std::env::var("SMTP_PORT").expect("SMTP_PORT must be set");
        Config {
            from,
            user,
            password,
            host,
            port: port.parse::<u16>().unwrap(),
        }
    }
}
