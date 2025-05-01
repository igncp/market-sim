FROM rust:1-slim-bookworm AS chef
RUN cargo install cargo-chef
RUN apt update && apt install -y \
  libssl-dev \
  openssl \
  pkg-config \
  ca-certificates
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY src src
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt update && apt install -y \
  libssl-dev \
  ca-certificates
WORKDIR /app
COPY scripts/docker scripts/docker
COPY --from=builder /app/target/release/market-sim .
CMD ["bash", "scripts/docker/run.sh"]
