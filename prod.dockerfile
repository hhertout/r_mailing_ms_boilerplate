FROM rust:1.74.0 as builder

WORKDIR /app

RUN apt-get -y update
RUN apt-get -y upgrade


COPY . .

RUN cargo build --release

FROM ubuntu:22.04

RUN apt-get -y update
RUN apt-get -y upgrade
RUN apt-get install -y sqlite3 libsqlite3-dev

WORKDIR /app

RUN mkdir data

COPY --from=builder /app/target/release/rust_mailer .
COPY ./templates ./templates

CMD ["./rust_mailer"]