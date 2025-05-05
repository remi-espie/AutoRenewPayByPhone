FROM rust:slim AS builder

ARG API_URL
ENV API_URL=$API_URL

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates gcc libssl-dev pkg-config perl make curl

RUN rustup target add wasm32-unknown-unknown

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

RUN cargo binstall dioxus-cli

COPY src ./src
COPY assets ./assets
COPY ./Cargo.toml .
COPY ./Dioxus.toml .

RUN cargo check

RUN dx build --platform web --release

FROM bitnami/nginx AS production

COPY --from=builder /usr/src/app/target/dx/front/release/web/public /app
