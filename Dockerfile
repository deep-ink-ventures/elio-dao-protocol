FROM rust:1.71.0-slim

RUN useradd builder
COPY . /home/builder
WORKDIR /home/builder

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked --version 0.9.1 soroban-cli

RUN ./init.sh

