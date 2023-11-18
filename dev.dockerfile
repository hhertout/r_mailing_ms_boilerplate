FROM rust:1.74

RUN cargo install cargo-watch

RUN apt-get -y update
RUN apt-get -y upgrade
RUN apt-get install -y sqlite3 libsqlite3-dev

WORKDIR /usr/src/app

COPY . .

CMD [ "cargo", "watch", "-x", "run" ] 