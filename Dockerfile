FROM rust:1.32.0-stretch as builder

WORKDIR /src
COPY ./ ./

RUN cargo build --release --locked

FROM debian:stretch

RUN apt-get update \
    && apt-get install -y ca-certificates

COPY --from=builder /src/target/release/gcp-access-token /usr/bin/gcp-access-token
