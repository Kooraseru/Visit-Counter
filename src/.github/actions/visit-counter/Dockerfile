FROM rust:1.89.0-bookworm AS builder
WORKDIR /build/rust
COPY rust/Cargo.toml ./
COPY rust/Cargo.lock ./
COPY rust/src ./src
RUN cargo test --locked \
    && cargo build --locked --release

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install --no-install-recommends --yes ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/rust/target/release/github-visit-counter /usr/local/bin/github-visit-counter
COPY assets/digits /opt/github-visit-counter/digits
ENTRYPOINT ["/usr/local/bin/github-visit-counter"]
