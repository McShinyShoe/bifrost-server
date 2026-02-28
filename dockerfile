FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev build-base pkgconfig sqlite-dev ca-certificates
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest
RUN apk add --no-cache ca-certificates
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/bifrost-server /usr/local/bin/bifrost-server
ENTRYPOINT ["/usr/local/bin/bifrost-server"]