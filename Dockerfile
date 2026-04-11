# syntax=docker/dockerfile:1.6
ARG SERVICE=mqtrader
ARG PORT=8080

# ------------------------------
# 1️⃣ Build Stage
# ------------------------------
FROM rust:1.94.1-slim-bookworm AS builder

WORKDIR /app

ARG SERVICE
ENV SERVICE=${SERVICE}

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --workspace || true

COPY . .
RUN cargo build --release -p ${SERVICE}

# ------------------------------
# 2️⃣ Runtime Stage
# ------------------------------
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    tzdata \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

RUN mkdir -p /app/strategies /app/config

ENV STRATEGY_CONFIG=/app/config/libmq_demo_strategy.json \
    STRATEGY_LIB=/app/strategies/libmq_demo_strategy

COPY --from=builder /app/target/release/${SERVICE} /usr/local/bin/${SERVICE}
COPY ~/.ssh /root/.ssh
RUN chmod 700 /root/.ssh && chmod 600 /root/.ssh/id_rsa
RUN ssh-keyscan github.com >> /root/.ssh/known_hosts
RUN ssh-agent bash -c 'ssh-add /root/.ssh/id_rsa'

ENV APP_ENV=production \
    RUST_BACKTRACE=1 \
    TZ=UTC

EXPOSE ${PORT}

ENTRYPOINT ["/usr/local/bin/mqtrader"]