FROM ubuntu:22.04

# --- Build-Time Arguments ---
ARG DEBIAN_FRONTEND=noninteractive
ARG SOLANA_CLI="1.18.18"
ARG ANCHOR_CLI="0.30.1"
ARG NODE_VERSION="18.16.0"

# --- Environment Variables ---
ENV HOME="/root"
ENV PATH="${HOME}/.cargo/bin:${PATH}"
ENV PATH="${HOME}/.local/share/solana/install/active_release/bin:${PATH}"

# --- Working Directory ---
WORKDIR /app

# --- Base Utilities ---
RUN apt-get update && apt-get install -y \
    build-essential git curl wget jq pkg-config python3-pip \
    libssl-dev libudev-dev ca-certificates && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# --- Install Node.js, npm, pnpm, and yarn ---
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -y nodejs && \
    npm install -g pnpm yarn

# --- Install Rust and Anchor CLI ---
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    rustup update stable && \
    rustup component add rustfmt clippy && \
    cargo install --git https://github.com/coral-xyz/anchor avm --locked && \
    avm install ${ANCHOR_CLI} && avm use ${ANCHOR_CLI}

# --- Install Solana CLI ---
RUN sh -c "$(curl -sSfL https://release.solana.com/v${SOLANA_CLI}/install)"

# --- Next.js Dependencies ---
COPY package.json pnpm-lock.yaml tsconfig.json ./
RUN pnpm install && pnpm add next@latest react@latest react-dom@latest

# --- Governance-UI Dependencies ---
WORKDIR /app/governance-ui
COPY governance-ui/package.json governance-ui/yarn.lock ./
RUN yarn install

# --- Build Frontend ---
WORKDIR /app
RUN pnpm build
WORKDIR /app/governance-ui
RUN yarn build

# --- Anchor Setup ---
WORKDIR /app/anchor
COPY anchor ./
RUN anchor build

# --- Interactive Shell ---
CMD ["/bin/bash"]