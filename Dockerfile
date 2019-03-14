FROM rust:1.33-slim-stretch

WORKDIR /onodejs

ADD Cargo.toml .
ADD Cargo.lock .
ADD src src

RUN apt-get update \
    && apt-get install -y libssl-dev openssl pkg-config \
    && rm -rf /var/lib/apt/lists/*
RUN cargo build --release
# && strip /onodejs/target/release/onodejs

FROM scratch
COPY --from=0 /onodejs/target/release/onodejs /bin/onodejs
ENTRYPOINT ["/bin/onodejs", "--prefix /bin", "use", "lts"]
