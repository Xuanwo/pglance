FROM postgres:16-bookworm

# Set noninteractive frontend for apt-get to prevent prompts
ENV DEBIAN_FRONTEND=noninteractive

# Install build dependencies, Rust, and cargo-pgrx

# Step 1: Update package lists
RUN apt-get update

# Step 2: Install system dependencies, including ca-certificates
RUN apt-get install -y --no-install-recommends \
    build-essential \
    curl \
    git \
    pkg-config \
    libssl-dev \
    clang \
    # Ensure libclang-dev matches the clang version if issues arise,
    # but for default clang, this should be fine.
    libclang-dev \
    ca-certificates \
    protobuf-compiler \
    libprotobuf-dev \
    cmake \
    postgresql-server-dev-16 \
    libgflags-dev \
    libsnappy-dev \
    libzstd-dev \
    liblz4-dev \
    libbrotli-dev

# Step 3: Install Rust and cargo-pgrx
# This combines Rust installation and cargo-pgrx installation into a single layer
# to ensure environment changes by rustup are effective for the cargo command.
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable && \
    # Source the cargo environment script to make cargo available in this RUN layer's shell
    . "$HOME/.cargo/env" && \
    # Install cargo-pgrx
    cargo install --locked cargo-pgrx@0.14.3

# Step 4: Set PATH environment variable to include Cargo's bin directory globally for subsequent layers
# This makes `cargo` and other Rust tools available for subsequent RUN commands and the final image.
ENV PATH="/root/.cargo/bin:${PATH}"

# Step 5: Set PGRX_HOME environment variable
ENV PGRX_HOME="/root/.pgrx"

# Step 6: Clean up apt cache to reduce image size
RUN apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Step 7: Set the working directory for pglance source code
WORKDIR /usr/src/pglance

# Copy the pglance project source code into the Docker image
# This assumes the Dockerfile is in the integration_tests/docker directory.
COPY ../../ .

# Step 8: Create PGRX_HOME directory
RUN mkdir -p ${PGRX_HOME}

# Step 9: Initialize pgrx for PostgreSQL 16
# This ensures PGRX_HOME is properly populated before cargo check
RUN cargo pgrx init --pg16 /usr/lib/postgresql/16/bin/pg_config

# Step 10: Run cargo check to catch compilation errors early
# Use the same features as the pgrx install step for consistency
RUN cargo check --no-default-features --features pg16

# Step 11: Compile/install the pglance extension.
# The pg_config path for postgres:16-bookworm is /usr/lib/postgresql/16/bin/pg_config.
# `cargo pgrx install` builds and installs the extension.
# ENV PATH should make 'cargo' available.
RUN cargo pgrx install --pg-config /usr/lib/postgresql/16/bin/pg_config

# Step 12: Copy the initialization script that will create the pglance extension in the database.
# Scripts in /docker-entrypoint-initdb.d/ are executed when the database is first initialized.
COPY ./init-pglance.sh /docker-entrypoint-initdb.d/init-pglance.sh

# Step 13: Make the initialization script executable
RUN chmod +x /docker-entrypoint-initdb.d/init-pglance.sh

# The default PostgreSQL Docker image entrypoint will handle starting the server.
# Our init-pglance.sh will be run automatically during the first startup.
# Default PostgreSQL port 5432 is exposed by the base image.
