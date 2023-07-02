# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the source code to the container
COPY . .

# Build the dependencies separately to improve build caching
RUN cargo build --release

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

# Build the application
CMD ["cargo", "run", "--release"]

