# Build stage
FROM rust:1.90 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Install Dioxus CLI
RUN cargo install cargo-binstall && \
    cargo binstall dioxus-cli --no-confirm

# Build the frontend
RUN dx build --release

# Runtime stage
FROM nginx:alpine

# Copy built frontend files
COPY --from=builder /app/dist /usr/share/nginx/html

# Expose port
EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]