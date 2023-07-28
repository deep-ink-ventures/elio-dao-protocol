FROM rust:latest 

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked --version 0.9.1 soroban-cli

COPY . .
RUN ./init.sh

# this requires a configured .env file in root
#RUN ./deploy.sh

