FROM rust:1.75-bookworm as builder
WORKDIR /app
COPY . .
RUN cargo build --release -p constellation-hub-mock

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/constellation-hub-mock /usr/local/bin/
EXPOSE 9001
CMD ["constellation-hub-mock"]
