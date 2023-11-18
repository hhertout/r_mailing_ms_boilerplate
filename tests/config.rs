use std::env;
use rust_mailer::services::mailer::config::Config;

#[test]
pub fn config_new() {
    let from = "noreply@test.fr";
    let host = "localhost";
    let port = "25";
    let user = "toto";
    let password = "password";
    let from_name = "EXAMPLE";
    env::set_var("SMTP_FROM", from);
    env::set_var("SMTP_FROM_NAME", from_name);
    env::set_var("SMTP_HOST", host);
    env::set_var("SMTP_PORT", port);
    env::set_var("SMTP_USER", user);
    env::set_var("SMTP_PASSWORD", password);

    let config = Config::new();

    assert_eq!(config.from, from);
    assert_eq!(config.user, user);
    assert_eq!(config.password, password);
    assert_eq!(config.host, host);
    assert_eq!(config.port, port.parse::<u16>().unwrap());
    assert_eq!(config.from_name, Some(from_name.to_owned()));
}

#[test]
pub fn config_new_without_from_name() {
    let from = "noreply@test.fr";
    let host = "localhost";
    let port = "25";
    let user = "toto";
    let password = "password";
    env::remove_var("SMTP_FROM_NAME");
    env::set_var("SMTP_FROM", from);
    env::set_var("SMTP_HOST", host);
    env::set_var("SMTP_PORT", port);
    env::set_var("SMTP_USER", user);
    env::set_var("SMTP_PASSWORD", password);

    let config = Config::new();
    assert_eq!(config.from, from);
    assert_eq!(config.user, user);
    assert_eq!(config.password, password);
    assert_eq!(config.host, host);
    assert_eq!(config.port, port.parse::<u16>().unwrap());
    assert_eq!(config.from_name, None);
}
