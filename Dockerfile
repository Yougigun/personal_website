# Start from the latest Rust base image
FROM rust:1.70.0 as builder

# Create a new empty shell project
WORKDIR /usr/src/app

# Copy our source code
COPY ./ ./

# Build for release
RUN cargo build --release

# Start a new stage
FROM debian:buster-slim

# Copy the binary from builder to this new stage
COPY --from=builder /usr/src/app/target/release/personal_website /usr/local/bin

# Port
EXPOSE 8080

# Command to run the binary
ENTRYPOINT ["personal_website"]