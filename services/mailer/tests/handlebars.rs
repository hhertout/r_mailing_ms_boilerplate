use handlebars::Handlebars;
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
    let result = handlebars
        .register_template_file("test", "./tests/templates/test.hbs");

    assert!(result.is_ok());
}

// TODO specifiq use case test for mailer