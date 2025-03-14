# Build stage for Rust
FROM rust:1.75-slim-bullseye as rust-builder

WORKDIR /usr/src/app
COPY . .

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Build the Rust application
RUN cargo build --release

# Build stage for Node.js
FROM node:20-slim as node-builder

WORKDIR /usr/src/app
COPY . .

# Install Node.js dependencies
RUN npm install

# Final stage
FROM debian:bullseye-slim

WORKDIR /usr/local/bin

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

# Copy the built Rust binary
COPY --from=rust-builder /usr/src/app/target/release/ai-trading-bot /usr/local/bin/

# Copy the Node.js application files
COPY --from=node-builder /usr/src/app/app.js /usr/local/bin/
COPY --from=node-builder /usr/src/app/style.css /usr/local/bin/
COPY --from=node-builder /usr/src/app/index.html /usr/local/bin/

# Set environment variables
ENV RUST_LOG=info
ENV NODE_ENV=production

# Expose the port your application uses
EXPOSE 3000

# Start the application
CMD ["ai-trading-bot"] 