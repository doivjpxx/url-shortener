# Use a Rust base image with Cargo installed
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock .env ./

RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build the dependencies without the actual source code to cache dependencies separately
RUN cargo build --release

# Copy the source code
COPY src src

# Build your application
RUN touch src/main.rs
RUN cargo build --release

# Start a new stage to create a smaller image without unnecessary build dependencies
FROM debian:bookworm-slim

# Set the working directory
WORKDIR /app

# Copy the built binary from the previous stage
COPY --from=builder /usr/src/app/target/release/url-shortener ./

# Expose the port the application runs on
EXPOSE 3000

# Command to run the application
CMD ["./url-shortener"]