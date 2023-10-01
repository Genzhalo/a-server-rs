FROM rust:latest AS builder

WORKDIR /myapp
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*

WORKDIR /myapp
COPY --from=builder /myapp/target/release/a-server .
COPY --from=builder /myapp/src src
COPY --from=builder /myapp/.env .

CMD ["./a-server"]