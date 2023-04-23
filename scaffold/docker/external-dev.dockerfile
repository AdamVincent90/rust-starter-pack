# This development image will aim to be as lightweight as possible.
# Especially to decrease cargo watch build time.

# Using official rust base image
FROM rust:latest

# Set the application directory
WORKDIR /external-api

# Install cargo-watch
RUN cargo install cargo-watch

# Copy the files to the Docker image
COPY ./ ./
