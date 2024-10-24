FROM rust:latest

WORKDIR /usr/src/portfolio

# deps
RUN apt install git && \
  cargo install typst-cli && \
  rustup toolchain install nightly && \
  rustup target add wasm32-unknown-unknown --toolchain nightly

COPY . .

RUN cargo install trunk && trunk build

RUN apt clean

EXPOSE 8000

CMD trunk serve

