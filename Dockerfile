FROM rust:slim AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim

RUN useradd -r -u 10001 -g nogroup appuser
WORKDIR /app
COPY --from=builder /app/target/release/rust-k8s-app-template /app/rust-k8s-app-template
USER appuser
EXPOSE 8080
ENTRYPOINT ["/app/rust-k8s-app-template"]
