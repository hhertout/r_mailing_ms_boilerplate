# Rust API microservice for mailing

Develop by myself.
## Getting Started

Don't forget to add your <code>.env</code> at the root with your SMTP logs.

### 1 - Create the template

Go to <code>services/mailer/templates</code>

Here, you will find two folder : 

- Layout : Correspond to the base layout of your template.
- Partials : Contain the style relative of the template you want to create.

At the root of the folder, is located the main template you want to create. 

### 2 - Connect it

Go to <code>src/api/mailer</code>, you will find an example of an Api route and handler made to generate template with custom data, and then to send email.

#### Example :

````rust
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
struct Req {
    to: String,
    subject: String,
    data: Data,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ResponseSuccess {
    message: String,
}

#[post("/ping")]
async fn hello_world(
    state: web::Data<AppState>,
    request: web::Json<Req>,
) -> Result<impl Responder> {
    let result = state
        .mailer
        .send_email(
            request.to.to_owned(),
            request.subject.to_owned(),
            String::from("helloworld"),
            request.data.to_owned(),
        )
        .await;

    match result {
        Ok(()) => Ok(web::Json(ResponseSuccess {message: String::from("Email successfully sent")})),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}
````

## Start with docker

Two variant has been added.

- One for development - it rebuild and restart the api at file change (if file is located in src folder).

- One for production - it install the app on the docker and start the executable.

To start docker, just run the following command :
- ```docker docker compose up```

## Stack

- Actix
- Letter
- Handlebar
- Tokio
- dotenvy

## Dev specs

#### Requirements:
- Dev -> Node js & npm

To watch change in your code, run <code>npx nodemon --watch src -e rs --exec cargo run</code>.
