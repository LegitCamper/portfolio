FROM rust:latest

WORKDIR /usr/src/portfolio

EXPOSE 8000

# deps
RUN apt install git && \
  cargo install typst-cli && \
  rustup toolchain install nightly && \
  rustup target add wasm32-unknown-unknown --toolchain nightly

COPY . .

RUN cargo install trunk && trunk build

CMD trunk serve

