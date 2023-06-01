# Use the official Rust image as the parent image
FROM rust:latest

# Set the working directory to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . /app

# Build the Rust application
RUN sudo apt update && sudo apt upgrade -y
RUN sudo apt install -y protobuf-compiler libprotobuf-dev
RUN cargo build

# Expose port 8000 for the application
EXPOSE 3000

# Set the startup command to run the application
CMD ["./target/debug/axum-web"]