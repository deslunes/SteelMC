FROM rust:1-alpine3.22 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin steel

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/steel .
RUN mkdir -p config
COPY --from=builder /app/package-content/favicon.png config/favicon.png
EXPOSE 25565
CMD ["./steel"]
