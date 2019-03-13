FROM rust:1.33-slim-stretch

WORKDIR /Downloads

ADD . .

RUN cargo build --release

CMD onodejs
