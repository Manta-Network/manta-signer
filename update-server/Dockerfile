FROM rust:1.58 as builder

RUN mkdir -p /opt/build

ADD . /opt/build

RUN cd /opt/build && cargo b --release

FROM ubuntu:focal
ENV RUST_LOG=info
WORKDIR /app

RUN apt-get update && apt-get install -y openssl ca-certificates

COPY --from=builder /opt/build/target/release/update-server /app
COPY .env /app

ENTRYPOINT ./update-server
