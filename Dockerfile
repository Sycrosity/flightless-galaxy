FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /app

RUN apt-get update && apt-get install -y g++ pkg-config libx11-dev libasound2-dev libudev-dev 
RUN cargo install basic-http-server
RUN cargo install cargo-watch
# RUN cargo install wasm-bindgen-cli

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN rustup target install wasm32-unknown-unknown
RUN cargo chef cook --release --recipe-path recipe.json --target wasm32-unknown-unknown

# Build application
COPY . .

RUN cargo build --release --target wasm32-unknown-unknown
RUN /app/tools/wasm-bindgen --out-dir ./dist/ --target web ./target/wasm32-unknown-unknown/release/flightless-galaxy.wasm

# FROM debian:buster-slim AS runtime
FROM rust:latest AS runtime
WORKDIR /app
COPY --from=builder /app/dist/ /app/
COPY ./assets /app/assets/ 
# COPY --from=chef /usr/local/cargo/bin/wasm-bindgen /usr/local/cargo/bin/wasm-bindgen
# COPY --from=chef /usr/local/cargo/bin/cargo-watch /usr/local/cargo/bin/cargo-watch
#copy the executable to run the http server
COPY --from=chef /usr/local/cargo/bin/basic-http-server /usr/local/cargo/bin/basic-http-server
ENTRYPOINT [ "/usr/local/cargo/bin/basic-http-server", "-a=0.0.0.0:80"]
