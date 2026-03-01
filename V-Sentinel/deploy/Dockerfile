# Multi-stage Dockerfile for SENTINEL Security System

# Stage 1: Build
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    clang \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /build

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

# Build the project
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 sentinel

# Create directories
RUN mkdir -p /var/lib/sentinel \
    /var/cache/sentinel \
    /var/log/sentinel \
    /etc/sentinel \
    && chown -R sentinel:sentinel /var/lib/sentinel \
    /var/cache/sentinel \
    /var/log/sentinel \
    /etc/sentinel

# Copy binary from builder
COPY --from=builder /build/target/release/sentinel /usr/local/bin/sentinel

# Copy configuration
COPY config/sentinel.toml /etc/sentinel/sentinel.toml

# Set permissions
RUN chmod +x /usr/local/bin/sentinel

# Switch to non-root user
USER sentinel

# Set working directory
WORKDIR /home/sentinel

# Expose ports
EXPOSE 8080 9090

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the application
CMD ["/usr/local/bin/sentinel", "--config", "/etc/sentinel/sentinel.toml"]