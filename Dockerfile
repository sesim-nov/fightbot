# First stage: Build the Rust binary
FROM rust AS builder

RUN apt update -y && apt upgrade -y && apt install protobuf-compiler -y

# Set working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY Cargo.toml Cargo.lock ./


# Copy the source code into the container
COPY . .

# Build the binary
RUN cargo build --release

# Second stage: Create the final minimal image
FROM debian:bookworm-slim
RUN apt update -y && apt upgrade -y
    #apt install openssl libssl-dev build-essential ca-certificates -y \
    #&& update-ca-certificates ...We don't need SSL, yet. 
# Set working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/team_queue_bot /app/team_queue_bot

# Run the binary
ENTRYPOINT ["./team_queue_bot"]
