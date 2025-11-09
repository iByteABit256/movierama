FROM rust:1.91 AS builder

WORKDIR /app
COPY . .

# Enable SQLx offline mode during build
ENV SQLX_OFFLINE=true

RUN cargo install --path .

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates

WORKDIR /app

COPY --from=builder /usr/local/cargo/bin/movierama /app/backend

ENV RUST_LOG=info
ENV DATABASE_URL=postgres://postgres:postgres@postgres:5432/movierama_db
EXPOSE 9000

CMD ["/app/backend"]

