FROM pandoc/typst AS resume-builder

WORKDIR /work
COPY resume .

RUN typst compile resume.typ --input admin=true --input developer=true

# --------------------

FROM rust:1.90.0-alpine AS arcade-builder

RUN apk add --no-cache curl gcc \
  libc-dev pkgconf \
  libx11-dev alsa-lib-dev \
  eudev-dev bash

RUN rustup target add wasm32-unknown-unknown

RUN cargo install wasm-bindgen-cli --version 0.2.117

WORKDIR /work

COPY build-games.sh .
COPY Cargo.toml .
COPY site ./site
COPY games ./games

RUN mkdir -p /work/site/public/arcade
RUN ./build-games.sh snake

# --------------------

FROM rust:1.90.0-alpine AS site-builder

RUN apk add --no-cache build-base

WORKDIR /work
COPY site .

RUN cargo build --release

# --------------------

FROM alpine:latest AS runner

WORKDIR /app
COPY --from=site-builder /work/target/release/site /app/site
COPY --from=site-builder /work/public /app/public
COPY --from=resume-builder /work/resume.pdf /app/public/assets/resume.pdf
COPY --from=arcade-builder /work/site/public/arcade/* /app/public/arcade/

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

CMD ["/app/site"]
