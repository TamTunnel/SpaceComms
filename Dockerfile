# Build stage
FROM rust:1.75-bookworm as builder

WORKDIR /app

# Copy workspace files
COPY Cargo.toml ./
COPY spacecomms-core ./spacecomms-core
COPY spacecomms-adapters ./spacecomms-adapters
COPY tests ./tests

# Build release
RUN cargo build --release -p spacecomms

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -r -u 1000 spacecomms

# Copy binary
COPY --from=builder /app/target/release/spacecomms /usr/local/bin/

# Copy default config
COPY examples/config.yaml /etc/spacecomms/config.yaml

# Set ownership
RUN chown -R spacecomms:spacecomms /etc/spacecomms

USER spacecomms

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s \
    CMD curl -f http://localhost:8080/health || exit 1

ENTRYPOINT ["spacecomms"]
CMD ["start", "--config", "/etc/spacecomms/config.yaml"]
