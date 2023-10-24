FROM rust:slim AS builder

RUN mkdir /app

WORKDIR /app

COPY Cargo.lock Cargo.toml ./
COPY src ./src
COPY benches ./benches

ENV RUSTFLAGS="-C target-cpu=native"

RUN [ "cargo", "build", "--release" ]

FROM debian:stable-slim

COPY --from=builder /app/target/release/main ./main

CMD ./main
