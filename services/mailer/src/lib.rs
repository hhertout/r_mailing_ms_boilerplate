use core::panic;

use crate::config::Config;
use handlebars::Handlebars;
use handlebars::RenderError;
use lettre::message::header::ContentType;
use lettre::AsyncTransport;
use lettre::Message;
use lettre::{transport::smtp::authentication::Credentials, AsyncSmtpTransport, Tokio1Executor};

pub mod config;

pub struct EmailInfo {
    to: String,
    subject: String,
    template_name: String,
}

pub struct Mailer {
    pub config: Config,
}

impl Mailer {
    pub fn new() -> Mailer {
        Mailer {
            config: Config::new(),
        }
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
        _data: E,
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

    pub async fn send_email<E>(
        &self,
        info: EmailInfo,
        data: E,
    ) -> Result<(), lettre::transport::smtp::Error> {
        let html_template = self
            .render_templates(&info.subject, data, &info.template_name)
            .unwrap_or_else(|_| panic!("Failed to parse template"));
        let email = Message::builder()
            .to(format!("<{}>", &info.to).parse().unwrap())
            .subject(info.subject)
            .header(ContentType::TEXT_HTML)
            .body(html_template)
            .unwrap_or_else(|_| panic!("Failed to create message"));
        let transporter = self.new_transporter();
        match transporter {
            Ok(t) => {
                t.send(email).await.unwrap();
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
