use crate::api::AppState;
use actix_web::{error::ErrorInternalServerError, post, web, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Data {
    title: String,
    text: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Request {
    to: String,
    subject: String,
    data: Data,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ResponseSuccess {
    status: u8,
    message: String,
}

#[post("/ping")]
async fn hello_world(
    state: web::Data<AppState>,
    request: web::Json<Request>,
) -> Result<impl Responder> {
    let result = state
        .mailer
        .send_email(
            state.db_pool.clone(),
            request.to.to_owned(),
            request.subject.to_owned(),
            String::from("helloworld"),
            request.data.to_owned(),
        )
        .await;

    match result {
        Ok(()) => Ok(web::Json(ResponseSuccess {
            status: 200,
            message: String::from("Email successfully sent"),
        })),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}
