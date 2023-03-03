FROM rust:slim

RUN mkdir /app

WORKDIR /app

COPY Cargo.lock Cargo.toml ./
COPY src ./src
COPY benches ./benches

ENV RUSTFLAGS="-C target-cpu=native"

RUN [ "cargo", "build", "--release" ]

CMD [ "cargo", "run", "--release" ]
