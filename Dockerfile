FROM debian:trixie-slim AS base

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      ca-certificates \
      libssl3 && \
    rm -rf /var/lib/apt/lists/*

FROM rust:slim-trixie AS builder

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      libssl-dev \
      pkg-config && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

RUN cargo build --release --no-default-features

FROM base AS runtime

COPY --from=builder /app/target/release/project-absence /usr/local/bin/project-absence

ENTRYPOINT ["project-absence"]
