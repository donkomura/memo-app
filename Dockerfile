FROM rust:1.89-slim AS builder

RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt/lists,sharing=locked \
    apt-get update && apt-get install -y --no-install-recommends \
      pkg-config \
      libssl-dev \
      ca-certificates

WORKDIR /app

RUN cargo install sqlx-cli --features postgres

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY . .
RUN cargo build --release --features postgres --no-default-features

FROM debian:bookworm-slim

RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt/lists,sharing=locked \
    apt-get update && apt-get install -y --no-install-recommends \
      ca-certificates \
      libssl3 \
      postgresql-client

RUN useradd -m -u 1000 appuser
WORKDIR /app

COPY --from=builder /app/target/release/memo-app /app/memo-app
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
COPY db /app/db
COPY docker-entrypoint.sh /app/docker-entrypoint.sh

RUN chown -R appuser:appuser /app && chmod +x /app/docker-entrypoint.sh
USER appuser

EXPOSE 8080
ENV RUST_LOG=info

ENTRYPOINT ["/app/docker-entrypoint.sh"]
CMD ["/app/memo-app"]
