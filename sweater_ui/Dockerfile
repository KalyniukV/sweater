# Use Rust Alpine as base
FROM rust:1.84-alpine AS builder

# Install dependencies
RUN apk add --no-cache musl-dev musl-utils libc-dev gcc \
    openssl-dev pkgconfig nodejs npm

# Install Rust WebAssembly target
RUN rustup target add wasm32-unknown-unknown

# Install Trunk and wasm-bindgen-cli
RUN cargo install trunk wasm-bindgen-cli

# Set working directory
WORKDIR /app

# Copy dependencies first to leverage Docker cache
COPY Cargo.toml Cargo.lock /app/

# Ensure src directory exists before copying
RUN mkdir -p /app/src

# Copy source code
COPY src /app/src/

# Copy frontend assets
COPY index.html /app/
COPY Trunk.toml /app/

# Check if files exist (for debugging)
RUN ls -l /app/

# Build the Yew application
RUN trunk build --release

# Use Nginx to serve the app
FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
