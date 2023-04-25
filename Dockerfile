# Stage 1: Build the application
FROM rust:1.69.0 as builder
WORKDIR /app

# Copy the source code and dependencies
COPY . .

# Build the application
RUN cargo build --release

# Stage 2: Create the final image GLIB_2.29
FROM debian:bullseye-slim
WORKDIR /app

# Install necessary dependencies
RUN apt-get update && \
    apt-get install -y libssl1.1 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/flashcard_master /app/flashcard

# Set environment variables
ENV RUST_LOG=info

# Start the application
ENTRYPOINT  ["/app/flashcard"]
