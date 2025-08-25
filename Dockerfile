FROM rust:alpine as arcade-builder

RUN apk add gcc libc-dev pkgconf libx11-dev alsa-lib-dev eudev-dev

RUN rustup +stable target add wasm32-unknown-unknown

RUN cargo install wasm-bindgen-cli

WORKDIR /work
COPY . .

RUN ./build-games.sh

FROM rustlang/rust:nightly-alpine as site-builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen git typst

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

RUN cargo +nightly leptos build --release -vv

FROM alpine:latest as runner

WORKDIR /app

COPY --from=site-builder /work/target/release/portfolio /app/
COPY --from=site-builder /work/target/site /app/site
COPY --from=site-builder /work/Cargo.toml /app/
COPY --from=arcade-builder /work/out/* /app/site/public/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8000"
ENV LEPTOS_SITE_ROOT=./site
EXPOSE 8000

CMD ["/app/portfolio"]
