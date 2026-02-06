FROM rust:1.92-slim AS builder
WORKDIR /app
COPY ./Cargo.* .
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /usr/local/bin/
COPY --from=builder /app/target/release/rc .
RUN chmod +x /usr/local/bin/rc
