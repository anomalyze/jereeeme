FROM rust:slim-bullseye as builder

WORKDIR app
COPY . .
# We need static linking for musl
RUN apt-get update && apt-get install -y musl-tools && rustup target add x86_64-unknown-linux-musl
# `cargo build` doesn't work in static linking, need `cargo install`
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/blog .
COPY ./assets ./assets
COPY ./articles ./articles
COPY ./static ./static
COPY ./templates ./templates

EXPOSE 8080

CMD ["./blog"]
