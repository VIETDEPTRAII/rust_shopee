# Base Rust image
FROM rust:1.82.0

# Install netcat for database connection checking
RUN apt-get update && apt-get install -y netcat-traditional && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /usr/src/app

# Install development tools
RUN cargo install cargo-watch
RUN cargo install sqlx-cli --no-default-features --features mysql

# Copy project files
COPY . .

# Install dependencies (optimized for hot reload)
RUN cargo build

# Expose the app port
EXPOSE 8080

# Command for hot reloading
CMD ["cargo", "watch", "-x", "run"]
