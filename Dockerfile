FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /app

ENV CARGO_WEB_HOST_TRIPLE="x86_64-unknown-linux-musl"
ENV CARGO_MAKE_VERSION="0.36.3"
ENV CARGO_WEB_URL="https://github.com/sagiegurari/cargo-make/releases/download/${CARGO_MAKE_VERSION}/cargo-make-v${CARGO_MAKE_VERSION}-${CARGO_WEB_HOST_TRIPLE}.zip"

RUN apt-get update && apt-get install -y g++ pkg-config libx11-dev libasound2-dev libudev-dev
RUN cargo install wasm-server-runner

RUN cargo install cargo-watch
RUN cargo install cargo-make
# RUN cargo install wasm-bindgen-cli

FROM chef AS planner
COPY . .
# RUN rustup toolchain install nightly
RUN cargo chef prepare --recipe-path recipe.json

# Build dependencies - this is the caching Docker layer!
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# RUN rustup toolchain install nightly
RUN rustup target install wasm32-unknown-unknown
# RUN cargo chef cook --release --recipe-path recipe.json --target wasm32-unknown-unknown
RUN cargo chef cook --recipe-path recipe.json --target wasm32-unknown-unknown --profile wasm-dev


# Build application
COPY . .
RUN cargo build --profile wasm-dev --target wasm32-unknown-unknown

# FROM debian:buster-slim AS runtime
FROM rust:latest AS runtime
WORKDIR /app
COPY --from=builder /app/wasm/ /app/
COPY ./assets /app/assets/ 
# COPY --from=chef /usr/local/cargo/bin/wasm-bindgen /usr/local/cargo/bin/wasm-bindgen
# COPY --from=chef /usr/local/cargo/bin/cargo-watch /usr/local/cargo/bin/cargo-watch
#copy the executable to run the http server
ENTRYPOINT [ "cargo", "make", "--makefile Makefile.toml", "watch-wasm" ]
