use crate::api::AppState;
use actix_web::web::Json;
use actix_web::{error::ErrorInternalServerError, get, web};
use logs::Logs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

#[get("/mailer/logs")]
async fn get_mailer_logs(state: web::Data<AppState>) -> Result<Json<Vec<Logs>>, actix_web::Error> {
    let res = state.mailer_logs.get_logs().await;
    match res {
        Ok(data) => Ok(web::Json(data)),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}
