FROM rust:latest-alpine

RUN cargo install cargo-watch

RUN apk --update-cache add sqlite

WORKDIR /usr/src/app

COPY . .

CMD [ "cargo", "watch", "-x", "run" ] 