use core::panic;
use sqlx::{Pool, Sqlite};
use crate::services::logs::{LogsRequest, MailerLogs};
use crate::services::mailer::config::Config;
use chrono::Local;
use lettre::message::header::ContentType;
use lettre::Address;
use lettre::message::Mailbox;
use lettre::Message;
use handlebars::Handlebars;
use handlebars::RenderError;
use lettre::Tokio1Executor;
use lettre::AsyncSmtpTransport;
use lettre::transport::smtp::authentication::Credentials;
use lettre::AsyncTransport;


pub mod config;

pub struct Mailer {
    pub config: Config,
    pub template_path: String,
}

impl Mailer {
    pub fn new(template_path: Option<String>) -> Mailer {
        Mailer {
            config: Config::new(),
            template_path: match template_path {
                Some(path) => path,
                None => String::from("templates"),
            },
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

    // TODO - Check if data sent is valid

    pub fn render_templates<E>(&self, data: E, template_name: &str) -> Result<String, RenderError>
        where
            E: serde::ser::Serialize,
    {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_file(
                template_name,
                &format!("./{}/{}.hbs", self.template_path.as_str(), template_name),
            )
            .unwrap_or_else(|_| panic!("Error : Wrong template path"));
        handlebars
            .register_template_file(
                "styles",
                &format!("./{}/partials/styles.hbs", self.template_path),
            )
            .unwrap_or_else(|_| panic!("Error : Wrong path for style template"));
        handlebars
            .register_template_file(
                "base",
                &format!("./{}/layouts/base.hbs", self.template_path),
            )
            .unwrap_or_else(|_| panic!("Error : Wrong path for base layout path"));

        let template_data = serde_json::json!(data);
        let content_template = handlebars.render(template_name, &template_data);
        match content_template {
            Ok(c) => Ok(c),
            Err(e) => Err(e),
        }
    }

    pub async fn send_email<E>(
        &self,
        db_pool: Pool<Sqlite>,
        to: String,
        subject: String,
        template_name: String,
        template_data: E,
    ) -> Result<(), lettre::transport::smtp::Error>
        where
            E: serde::ser::Serialize,
    {
        let html_template = self
            .render_templates(template_data, &template_name)
            .unwrap();
        let email_template = Message::builder()
            .from(Mailbox::new(
                self.config.from_name.to_owned(),
                self.config.from.parse::<Address>().unwrap(),
            ))
            .to(Mailbox::new(None, to.as_str().parse::<Address>().unwrap()))
            .subject(subject.clone())
            .header(ContentType::TEXT_HTML)
            .body(html_template);
        let email = match email_template {
            Ok(email) => email,
            Err(_) => panic!("failed to build email"),
        };
        let transporter = self.new_transporter();
        match transporter {
            Ok(t) => {
                match t.send(email).await {
                    Ok(_) => {
                        let data = LogsRequest {
                            subject,
                            to,
                            date: Local::now().to_string(),
                            success: true,
                            error_desc: None
                        };
                        let _ = MailerLogs::new().await.insert_one(db_pool,&data).await;
                    }
                    Err(e) => {
                        let data = LogsRequest {
                            subject,
                            to,
                            date: Local::now().to_string(),
                            success: false,
                            error_desc: Some(e.to_string())
                        };
                        let _ = MailerLogs::new().await.insert_one(db_pool, &data).await;
                    }
                }
                Ok(())
            }
            Err(e) => {
                let data = LogsRequest {
                    subject,
                    to,
                    date: Local::now().to_string(),
                    success: false,
                    error_desc: Some(e.to_string())
                };
                let _ = MailerLogs::new().await.insert_one(db_pool,&data).await;
                Err(e)
            }
        }
    }
}
