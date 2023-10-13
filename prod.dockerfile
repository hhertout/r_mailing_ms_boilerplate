FROM rust:latest-alpine

WORKDIR /usr/src/app

RUN apk --update-cache add sqlite

COPY . .

RUN cargo install --path .

CMD ["rust_mailer"]