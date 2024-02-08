# Use the latest version of the Rust base image
FROM rust:latest

# Set the working directory in the container to /memomi (fuck yeah)
WORKDIR /usr/src/memomi

# Copy the Rust project files to the working directory
COPY /back_logic .

# Build the Rust app
RUN cargo build

# Open port with the hello world app (just GET "/")
EXPOSE 80

# Set the command to run the Rust app
CMD cargo run
