# Rust API microservice for mailing

Developed by myself.

Build with Sqlite database for logs.
## Getting Started

Don't forget to add your ```.env``` at the root with your SMTP logs.

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

#[post("/ping")] // define the endpoint
async fn hello_world(
    state: web::Data<AppState>, // get the global state
    request: web::Json<Req>, // get the entering request and deserialize it front Req struct
) -> Result<impl Responder> {
    let result = state
        .mailer
        .send_email(
            request.to.to_owned(), // to
            request.subject.to_owned(), // subject
            String::from("helloworld"), // template name
            request.data.to_owned(), // template data
        )
        .await;

    match result {
        Ok(_) => Ok(web::Json(ResponseSuccess {message: String::from("Email successfully sent")})),
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

## Dev specs

#### Requirements:

- Without docker : Node js & npm

To run the app : 
- ```cargo run```

To watch change in your code, run 
- ```npx nodemon --watch src -e rs --exec cargo run```.
