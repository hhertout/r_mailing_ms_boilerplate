use crate::api::AppState;
use actix_web::{post, web, Responder, Result, error::ErrorInternalServerError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Req {
    to: String,
    subject: String,
    text: String,
}

#[post("/ping")]
async fn hello_world(
    state: web::Data<AppState>,
    request: web::Json<Req>,
) -> Result<impl Responder> {
    let result = state.mailer.send_email(
        request.to.to_owned(),
        request.subject.to_owned(),
        String::from("helloworld"),
        request.text.to_owned(),
    ).await;

    match result {
        Ok(()) => Ok(web::Json(request)),
        Err(e) => Err(ErrorInternalServerError(e))
    }
}
