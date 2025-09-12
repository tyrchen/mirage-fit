# Multi-stage build for smaller final image
# Stage 1: Build the Rust backend
FROM rust:1.75-bookworm AS backend-builder

WORKDIR /app

# Copy only the files needed for dependency resolution first
# This allows Docker to cache the dependency layer
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy the actual source code
COPY src ./src
COPY fixtures ./fixtures

# Build the actual application
RUN touch src/main.rs && \
    cargo build --release

# Stage 2: Build the React frontend
FROM node:20-bookworm AS frontend-builder

WORKDIR /app/ui

# Copy package files for dependency installation
COPY ui/package*.json ./
COPY ui/yarn.lock* ./

# Install dependencies
RUN npm ci || yarn install --frozen-lockfile

# Copy the rest of the frontend code
COPY ui/ ./

# Build the frontend
RUN npm run build || yarn build

# Stage 3: Final runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user to run the application
RUN useradd -m -u 1001 -s /bin/bash mirage

WORKDIR /app

# Copy the built binary from the backend builder
COPY --from=backend-builder /app/target/release/mirage-fit /app/mirage-fit

# Copy the built frontend from the frontend builder
COPY --from=frontend-builder /app/ui/dist /app/ui/dist

# Copy static files and fixtures
COPY fixtures /app/fixtures

# Create data directory for image storage
RUN mkdir -p /app/data && \
    chown -R mirage:mirage /app

# Switch to non-root user
USER mirage

# Environment variables
ENV PORT=3000
ENV DATA_DIR=/app/data
ENV RUST_LOG=info

# Expose the port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:3000/api/health || exit 1

# Run the application
CMD ["./mirage-fit"]
