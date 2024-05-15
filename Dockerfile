# Use an official Rust image from the Docker Hub
FROM rust:1.65 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin whistle
WORKDIR /whistle

# Copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# This trick will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Now that the dependencies are built, copy your source tree
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/whistle*
RUN cargo build --release

# Final base image
FROM debian:bullseye-slim
WORKDIR /root/

# Copy the build artifact from the build stage and remove extra files
COPY --from=builder /whistle/target/release/whistle .

# Install needed packages
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Ensure the binary is executable
RUN chmod +x ./whistle

# Expose the port the server is listening on
ENV PORT "8080"
ENV RUST_LOG info

# Use environment variables to pass into the application
ENV DISCORD_TOKEN ""
ENV DISCORD_CHANNEL_ID ""
ENV DISCORD_CLIENT_ID ""

# Command to run the executable
CMD ["./whistle"]
