FROM rust:slim AS builder

WORKDIR /usr/src/app

COPY src ./src
COPY Cargo.toml .

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates gcc libssl-dev pkg-config

RUN cargo build --release

FROM debian:bookworm-slim AS production

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates gcc libssl-dev

COPY --from=builder /usr/src/app/target/release/back /AutoRenewPayByPhone

CMD ["/AutoRenewPayByPhone"]
