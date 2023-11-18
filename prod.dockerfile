FROM rust:1.74.0

WORKDIR /usr/src/app

RUN apt-get -y update
RUN apt-get -y upgrade
RUN apt-get install -y sqlite3 libsqlite3-dev

COPY . .

RUN cargo install --path .

CMD ["rust_mailer"]