# Base build stage
FROM rust:1.78.0 AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    curl git gnupg unzip build-essential pkg-config \
    libssl-dev libreadline-dev zlib1g-dev libsqlite3-dev libbz2-dev libffi-dev liblzma-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /usr/src/app

# Copy and build the application
COPY . .
RUN cargo build --release --workspace

# Final runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy compiled binary and necessary files from the build stage
COPY --from=builder /usr/src/app/target/release/raesan /app/raesan
COPY --from=builder /usr/src/app/database/raesan_base.db /app/database/raesan_base.db
COPY ./entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

# Create a non-root user
RUN groupadd -r raesan-user && useradd -r -g raesan-user raesan-user
RUN chown -R raesan-user:raesan-user /app
USER raesan-user

# Expose port
EXPOSE 8080

ENTRYPOINT ["/app/entrypoint.sh"]
