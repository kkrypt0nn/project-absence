FROM debian:bookworm-slim AS base
RUN apt-get update && apt-get install -y libssl-dev ca-certificates pkg-config

FROM rust:slim-bookworm AS builder
COPY --from=base / /
WORKDIR /app
ADD . /app
RUN cargo build --release --no-default-features

FROM base
COPY --from=builder /app/target/release/project-absence /
ENTRYPOINT ["./project-absence"]
