FROM ubuntu:22.04

# Define environment variables
ARG NODE_VERSION="18.16.0"
ARG ANCHOR_VERSION="0.30.1"
ENV PATH="/root/.cargo/bin:$PATH"

WORKDIR /app

# Install base tools
RUN apt-get update && apt-get install -y \
    curl \
    git \
    build-essential \
    python3-pip \
    libssl-dev \
    libudev-dev \
    pkg-config \
    wget \
    jq && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Install Node.js and PNPM
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -y nodejs && \
    npm install -g pnpm

# Install Rust and Anchor
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    . "$HOME/.cargo/env" && \
    rustup update stable && \
    rustup component add rustfmt clippy && \
    cargo install --git https://github.com/coral-xyz/anchor avm --locked --version $ANCHOR_VERSION

# Copy package files first for better caching
COPY package.json pnpm-lock.yaml ./

# Install dependencies
RUN pnpm install
RUN pnpm add next@latest react@latest react-dom@latest

# Install and build
RUN pnpm update
RUN pnpm build


# Deploy in anchor/.devcontainer
WORKDIR /anchor/.devcontainer
RUN docker buildx build . --platform linux/amd64

# Copy project files
COPY . .

CMD ["/bin/bash"]
