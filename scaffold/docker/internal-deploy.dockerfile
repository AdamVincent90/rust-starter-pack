# This deployment image will aim to be as lightweight as possible and will most likely use distroless and build stages.

# Using official rust base image
FROM rust:latest

# Set the application directory
WORKDIR /internal-api

# Install cargo-watch
RUN cargo install cargo-watch

# Copy the files to the Docker image
COPY ./ ./

