# This deployment image will aim to be as lightweight as possible.

# Using official rust base image
FROM rust:latest

# Set the application directory
WORKDIR /rust-app

# Install cargo-watch
RUN cargo install cargo-watch

# Copy the files to the Docker image
COPY ./ ./