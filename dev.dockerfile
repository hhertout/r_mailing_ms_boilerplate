FROM rust:latest-alpine

RUN cargo install cargo-watch

WORKDIR /usr/src/app

COPY . .

CMD [ "cargo", "watch", "-x", "run" ] 