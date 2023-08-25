use crate::config::Config;
use handlebars::Handlebars;
use handlebars::RenderError;
use lettre::{transport::smtp::authentication::Credentials, AsyncSmtpTransport, Tokio1Executor};

pub mod config;

pub struct EmailData {
    text: String,
}

pub struct Mailer {
    config: Config,
}

impl Mailer {
    pub fn new() -> Mailer {
        Mailer { config: Config::new() }
    }

    pub fn new_transporter(
        &self,
    ) -> Result<AsyncSmtpTransport<Tokio1Executor>, lettre::transport::smtp::Error> {
        let creds = Credentials::new(self.config.user.to_owned(), self.config.password.to_owned());
        let transport =
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&self.config.host.to_owned());
        match transport {
            Ok(transporter) => Ok(transporter
                .port(self.config.port)
                .credentials(creds)
                .build()),
            Err(e) => Err(e),
        }
    }

    pub fn render_base_templates(&self, mut handlebars: Handlebars, template_name: &str) {
        handlebars
            .register_template_file(template_name, &format!("./templates/{}.hbs", template_name))
            .unwrap_or_else(|_| panic!("Wrong template path"));
        handlebars
            .register_template_file("styles", "./templates/partials/styles.hbs")
            .unwrap_or_else(|_| panic!("Error : Wrong path for style template"));
        handlebars
            .register_template_file("base", "./templates/layouts/base.hbs")
            .unwrap_or_else(|_| panic!("Error : Wrong path for base layout path"));
    }

    pub fn render_templates<E>(
        &self,
        subject: &str,
        _data: Option<E>,
        template_name: &str,
    ) -> Result<String, RenderError> {
        let handlebars = Handlebars::new();
        self.render_base_templates(handlebars.to_owned(), template_name);
        let template_data = serde_json::json!({
            "subject": subject.split_whitespace().next().unwrap(),
            "text": "Hello world from rust mailer",
        });
        let content_template = handlebars.render(template_name, &template_data);
        match content_template {
            Ok(c) => Ok(c),
            Err(e) => Err(e),
        }
    }
}
