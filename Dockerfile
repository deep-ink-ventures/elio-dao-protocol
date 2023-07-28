FROM ubuntu:22.04 as builder

RUN apt update && apt install -y curl build-essential
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rust_install.sh
RUN sh rust_install.sh -y

ENV PATH="$PATH:/root/.cargo/bin"

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked --version 0.9.1 soroban-cli

FROM builder as elio-dao

COPY . .
RUN ./init.sh

# this requires a configured .env file in root
RUN ./deploy.sh

