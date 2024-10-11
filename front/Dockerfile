FROM rust:alpine AS builder

WORKDIR /usr/src/app

COPY src ./src
COPY ./Cargo.toml .
COPY ./Cargo.lock .

RUN cargo install dioxus-cli

RUN rustup target add wasm32-unknown-unknown

RUN dx build --platform web --release

FROM bitnami/nginx AS production

COPY --from=builder /usr/src/app/dist /app
