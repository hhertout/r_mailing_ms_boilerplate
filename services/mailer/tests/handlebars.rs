use handlebars::Handlebars;
use mailer::Mailer;
use serde::Serialize;

// Base handlebar tests
#[test]
pub fn handlebars_base() {
    let mut handlebars = Handlebars::new();
    let source = "Hello {{world}}";
    assert!(handlebars.register_template_string("hello", source).is_ok());

    #[derive(Serialize)]
    struct Data {
        world: String,
    }
    let data = Data {
        world: String::from("World"),
    };
    assert!(handlebars.render("hello", &data).is_ok());
    assert_eq!(
        handlebars.render("hello", &data).unwrap(),
        "Hello World".to_owned()
    );
}

#[test]
pub fn handlebars_register_template_fail() {
    let mut handlebars = Handlebars::new();
    let result = handlebars.register_template_file("test", "wrong_url/test.hbs");
    assert!(result.is_err());
}

#[test]
pub fn handlebars_register_template_success() {
    let mut handlebars = Handlebars::new();
    let result = handlebars.register_template_file("test", "./tests/templates/test.hbs");

    assert!(result.is_ok());
}

#[test]
pub fn test_handlebar_render_mailer_template() {
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

    let template_path = String::from("tests/templates");
    let mailer = Mailer::new(Some(template_path));

    #[derive(Serialize)]
    struct Data {
        world: String,
    }
    let data = Data {
        world: String::from("World"),
    };
    let result = mailer.render_templates(data, "test");
    assert!(result.is_ok());
}
