<p align="center">
  <h1 align="center">Mailing MicroService</h1>
    <p align="center">Rust Mailing microservice boilerplate</p>
</p>

<p align="center">
    <img src="https://img.shields.io/github/v/release/hhertout/api_rust_mailer.svg" />
    <a href="https://github.com/hhertout/rac_tool/actions">
      <img alt="Tests Passing" src="https://github.com/hhertout/api_rust_mailer/actions/workflows/rust.yml/badge.svg" />
    </a>
</p>

## Features

- Expose API endpoint dedicated to the service
- Build email templates
- Send dynamic email from templates and values provided
- Sending history with sqlite database

## Getting Started

Before running the application, make sure to copy the `.env.example` file to a new file named `.env`. 
This file contains important configuration settings for the application. 

Be sure to customize the values in the `.env` file to match your environment and requirements. 

Keeping the `.env.example` file as a template helps ensure that your application has the necessary configuration variables in place.

### 1 - Create the template

Go to ```services/mailer/templates```

Here, you will find two folder : 

- Layout : Correspond to the base layout of your template.
- Partials : Contain the style relative of the template you want to create.

At the root of the folder, is located the main template you want to create. 

### 2 - Connect it

Go to ```src/api/mailer```, you will find an example of an Api route and handler made to generate template with custom data, and then to send email.

#### Example :

````rust
use crate::api::AppState;
use actix_web::{error::ErrorInternalServerError, post, web, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ErrorResponse { // define error response struct
    message: String, 
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Req { // define the request data structure
    to: String,
    subject: String,
    data: Data,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Data {
    title: String,
    text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ResponseSuccess { // define the response struct
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
            state.db_pool.clone(),
            request.to.to_owned(),
            request.subject.to_owned(),
            String::from("helloworld"),
            request.data.to_owned(),
        )
        .await;

    match result {
        Ok(()) => Ok(web::Json(ResponseSuccess {
            message: String::from("Email successfully sent"),
        })),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}
````
### Expose endpoint

Don't forget to put all your routes in the dedicated function :

```rust
fn router(cfg: &mut ServiceConfig) {
    // Put all your route here
    cfg.service(your_route);
}
```

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
- sqlx

## Developement process

To run the app : 
- ```cargo run```

To watch change in your code, run 
- ```cargo watch -x run```.

## Logs & Sqlite configuration

Logs are available with sqlx.
It saves all sending requests with the result.

### Migrations

Migration are located on ```/services/logs/migrations```.

To create a new migration, go on ```/services/logs/migrations``` and run 
```bash
sqlx migrate add <DESCRIPTION>
```

The migration files were executed at server startup. 