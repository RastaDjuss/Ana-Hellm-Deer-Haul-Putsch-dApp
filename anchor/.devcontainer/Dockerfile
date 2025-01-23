FROM ubuntu:22.04

# Définir les arguments de version
ARG DEBIAN_FRONTEND=noninteractive
ARG SOLANA_CLI="1.18.18"
ARG ANCHOR_CLI="0.30.1"
ARG NODE_VERSION="v18.16.0"

# Définir les environnements et chemins
ENV HOME="/root"
ENV PATH="${HOME}/.cargo/bin:${PATH}"
ENV PATH="${HOME}/.local/share/solana/install/active_release/bin:${PATH}"
ENV PATH="${HOME}/.nvm/versions/node/${NODE_VERSION}/bin:${PATH}"

# Installer les utilitaires de base pour tout le projet
RUN apt-get update -qq && apt-get upgrade -qq && apt-get install -qq \
    build-essential git curl wget jq pkg-config python3-pip libssl-dev libudev-dev && \
    rm -rf /var/lib/apt/lists/*

# Installer Node.js et PNPM pour la racine et les sous-répertoires
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh | bash
ENV NVM_DIR="$HOME/.nvm"
RUN . "$NVM_DIR/nvm.sh" && \
    nvm install ${NODE_VERSION} && \
    nvm use ${NODE_VERSION} && \
    nvm alias default node && \
    npm install -g pnpm

# Installer Rust, Anchor et Solana uniquement pour le dossier /anchor
RUN apt-get install -qq libssl-dev
RUN curl "https://sh.rustup.rs" -sfo rustup.sh && \
    sh rustup.sh -y && \
    rustup component add rustfmt clippy && \
    cargo install --git https://github.com/coral-xyz/anchor avm --locked --force

RUN sh -c "$(curl -sSfL https://release.solana.com/v${SOLANA_CLI}/install)" && \
    avm install ${ANCHOR_CLI} && avm use ${ANCHOR_CLI}

# Définir les chemins de travail
WORKDIR /workdir

# Note : Docker configure tantôt root (React), tantôt /anchor (Rust, Anchor)