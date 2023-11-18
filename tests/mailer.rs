use rust_mailer::services::mailer::Mailer;

#[test]
fn mailer_new() {
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

    let mailer = Mailer::new(None);

    assert_eq!(mailer.config.from, from);
    assert_eq!(mailer.config.user, user);
    assert_eq!(mailer.config.password, password);
    assert_eq!(mailer.config.host, host);
    assert_eq!(mailer.config.port, port.parse::<u16>().unwrap());
    assert_eq!(
        mailer.template_path,
        String::from("templates")
    );
}

#[test]
fn mailer_new_with_repo() {
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

    let template_path = String::from("template/test");
    let mailer = Mailer::new(Some(template_path));

    assert_eq!(mailer.config.from, from);
    assert_eq!(mailer.config.user, user);
    assert_eq!(mailer.config.password, password);
    assert_eq!(mailer.config.host, host);
    assert_eq!(mailer.config.port, port.parse::<u16>().unwrap());
    assert_eq!(mailer.template_path, String::from("template/test"));
}
