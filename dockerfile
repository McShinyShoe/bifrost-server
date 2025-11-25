FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest

WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/bifrost-server /usr/local/bin/bifrost-server

CMD ["bifrost-server"]
