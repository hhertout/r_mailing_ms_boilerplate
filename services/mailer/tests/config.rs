/* let from = std::env::var("SMTP_FROM").unwrap_or_else(|_| panic!("Failed to load env variable"));
let user = std::env::var("SMTP_USER").expect("STMP_USER must be set");
let password = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
let host = std::env::var("SMTP_HOST").expect("SMTP_HOST must be set");
let port = std::env::var("SMTP_PORT").expect("SMTP_PORT must be set"); */

use mailer::config::Config;

#[test]
pub fn env_from_test() {
    let test_from = "noreply@test.fr";
    std::env::set_var("SMTP_FROM", test_from);
    assert_eq!(std::env::var("SMTP_FROM").unwrap(), test_from.to_string());
}
#[test]
pub fn env_user_test() {
    let test_user = "test";
    std::env::set_var("SMTP_USER", test_user);
    assert_eq!(std::env::var("SMTP_USER").unwrap(), test_user.to_string());
}
#[test]
pub fn env_password_test() {
    let test_password = "test";
    std::env::set_var("SMTP_PASSWORD", test_password);
    assert_eq!(std::env::var("SMTP_PASSWORD").unwrap(), test_password.to_string());
}
#[test]
pub fn env_host_test() {
    let test_host = "localhost";
    std::env::set_var("SMTP_HOST", test_host);
    assert_eq!(std::env::var("SMTP_HOST").unwrap(), test_host.to_string());
}
#[test]
pub fn env_port_test() {
    let test_port = "25";
    std::env::set_var("SMTP_PORT", test_port);
    assert_eq!(std::env::var("SMTP_PORT").unwrap(), test_port.to_string());
}
#[test]
pub fn config_new() {
    let from = "noreply@test.fr";
    let host = "localhost";
    let port = "25";
    let user = "toto";
    let password = "password";
    std::env::set_var("SMTP_FROM", from);
    std::env::set_var("SMTP_HOST", host);
    std::env::set_var("SMTP_PORT", port);
    std::env::set_var("SMTP_USER", user);
    std::env::set_var("SMTP_PASSWORD", password);

    let config = Config::new();

    assert_eq!(config.from, from);
    assert_eq!(config.user, user);
    assert_eq!(config.password, password);
    assert_eq!(config.host, host);
    assert_eq!(config.port, port.parse::<u16>().unwrap());
}
