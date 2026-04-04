FROM pandoc/typst AS resume-builder

WORKDIR /work
COPY resume .

RUN typst compile resume.typ --input admin=true --input developer=true

# --------------------

FROM rust:1.90.0-alpine AS arcade-builder

RUN apk add curl gcc libc-dev pkgconf libx11-dev alsa-lib-dev eudev-dev

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | sh 

RUN rustup target add wasm32-unknown-unknown

RUN cargo binstall wasm-bindgen-cli --version 0.2.117

WORKDIR /work
COPY games games
COPY build-games.sh build-games.sh

RUN ./build-games.sh snake

# --------------------

FROM rust:1.90.0-alpine AS site-builder

RUN apk add curl make musl-dev  

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | sh 

WORKDIR /work
COPY site .

RUN rustup target add wasm32-unknown-unknown

RUN cargo binstall cargo-leptos --version 0.3.5

RUN cargo leptos build --release -vv

# --------------------

FROM alpine:latest AS runner

WORKDIR /app
COPY --from=site-builder /work/target/release/site /app/
COPY --from=site-builder /work/target/site /app/site
COPY --from=site-builder /work/Cargo.toml /app/
COPY --from=resume-builder /work/resume.pdf /app/site/public/assets/resume.pdf
COPY --from=arcade-builder /work/site/public/arcade/* /app/site/public/arcade/

CMD ["/app/site"]
