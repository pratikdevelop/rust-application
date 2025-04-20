# Build stage
FROM rust:1.82 AS builder

WORKDIR /app

# Copy the project files
COPY . .

# Build the release binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install ca-certificates for HTTPS support
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/api /usr/local/bin/api

# Expose port 8080
EXPOSE 8080

# Run the server
ENTRYPOINT ["api"]