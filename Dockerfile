FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build dependencies - this is the caching Docker layer!
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
#ensure that we are building for nightly wasm32 target by copying the wasm-toolchain
COPY ./wasm-toolchain.toml rust-toolchain.toml

# RUN rustup target install wasm32-unknown-unknown
RUN cargo chef cook --recipe-path recipe.json --target wasm32-unknown-unknown --profile wasm-dev

# Build application
RUN cargo install wasm-server-runner
COPY . .
RUN cargo build --profile wasm-dev --target wasm32-unknown-unknown

#final, size efficient docker layer
FROM debian:buster-slim AS runtime
WORKDIR /app

# RUN apt-get update ; apt-get install -y --no-install-recommends g++ pkg-config libx11-dev libasound2-dev libudev-dev ; rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/wasm32-unknown-unknown/wasm-dev/flightless-galaxy.wasm /app/flightless-galaxy.wasm
COPY --from=builder /usr/local/cargo/bin/wasm-server-runner /usr/bin/wasm-server-runner
COPY ./assets /app/assets

#copy the executable to run the standalone wasm file and runner
ENTRYPOINT ["WASM_SERVER_RUNNER_ADDRESS=0.0.0.0:80", "/usr/bin/wasm-server-runner", "/app/flightless-galaxy.wasm"]
