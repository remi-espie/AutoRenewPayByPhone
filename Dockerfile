FROM rust:alpine AS builder

WORKDIR /usr/src/app

COPY src ./src
COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo build --release

FROM scratch AS production

COPY --from=builder /usr/src/app/target/release/AutoRenewPayByPhone /AutoRenewPayByPhone

CMD ["/AutoRenewPayByPhone"]
