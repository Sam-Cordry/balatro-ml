FROM rust:1.85-slim

WORKDIR /app
COPY src/ ./src
COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo build --release

CMD ["./target/release/balatro-simulator"]
