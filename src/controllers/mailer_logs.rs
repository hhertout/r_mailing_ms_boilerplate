use actix_web::web::Json;
use actix_web::{error::ErrorInternalServerError, get, web};
use serde::{Deserialize, Serialize};
use crate::api::AppState;
use crate::services::logs::{Logs, MailerLogs};

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

#[get("/mailer/logs")]
async fn get_mailer_logs(state: web::Data<AppState>) -> Result<Json<Vec<Logs>>, actix_web::Error> {
    let res = MailerLogs::new().await.get_logs(state.db_pool.clone()).await;
    match res {
        Ok(data) => Ok(web::Json(data)),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}
