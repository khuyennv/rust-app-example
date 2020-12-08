FROM rust:1.47.0 AS builder
WORKDIR /
RUN USER=root cargo new app
WORKDIR /app
COPY . .
RUN cargo build -p main --release

# Main
FROM ubuntu:18.04
RUN apt-get update && apt-get install -y libssl-dev && apt install -y apt-transport-https ca-certificates curl software-properties-common
WORKDIR /app
COPY --from=builder /app/target/release/main .
CMD ["./api"]
