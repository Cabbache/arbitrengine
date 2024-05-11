# Stage 1: Build the Rust application
FROM rust:1.78-bullseye as builder
WORKDIR /usr/src/myapp

# Copy the source code into the container
COPY ./src .

# Build the application
RUN cargo build --release

# Stage 2: Set up the runtime environment
FROM debian:bullseye
WORKDIR /root/

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/myapp/target/release/arbitrengine .

# Install socat
RUN apt-get update && apt-get install -y socat && rm -rf /var/lib/apt/lists/*

# Command to run the binary with socat for stdin/stdout handling
CMD ["socat", "TCP-LISTEN:5000,reuseaddr,fork", "EXEC:/root/arbitrengine,pty,setsid,ctty,sigint"]
