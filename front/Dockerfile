FROM rust:slim AS builder

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates gcc libssl-dev pkg-config perl make

RUN rustup target add wasm32-unknown-unknown

RUN cargo install dioxus-cli

COPY src ./src
COPY assets ./assets
COPY ./Cargo.toml .
COPY ./Dioxus.toml .

RUN dx build --platform web --release

FROM bitnami/nginx AS production

COPY --from=builder /usr/src/app/dist /app
