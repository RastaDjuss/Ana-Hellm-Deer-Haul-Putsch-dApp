FROM ubuntu:22.04

# Définir l'environnement et les variables
ARG NODE_VERSION="18.16.0"
ARG ANCHOR_VERSION="0.30.1"

# Installer les outils de base
RUN apt-get update && apt-get install -y \
    curl \
    git \
    build-essential \
    python3-pip \
    libssl-dev

# Installer Node.js et PNPM pour le Frontend
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
RUN apt-get install -y nodejs
RUN npm install -g pnpm

# Installer Rust et Anchor pour `/anchor`
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="$HOME/.cargo/bin:$PATH"
RUN cargo install --git https://github.com/coral-xyz/anchor avm --locked --version $ANCHOR_VERSION

# Définir le dossier de travail
WORKDIR /workdir

# Copier les fichiers (racine et projets children)
COPY . /workdir