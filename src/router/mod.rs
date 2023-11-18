use actix_web::web::ServiceConfig;
use crate::controllers::mailer::hello_world;
use crate::controllers::mailer_logs::get_mailer_logs;

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(hello_world);
    cfg.service(get_mailer_logs);
}