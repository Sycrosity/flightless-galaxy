FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
COPY ./wasm/toolchain.toml rust-toolchain.toml
RUN cargo chef prepare --recipe-path recipe.json
#--bin flightless-galaxy
# RUN cargo chef prepare --bin wasm-server-runner --recipe-path wasm-server-runner-recipe.json

# Build dependencies - this is the caching Docker layer!
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# COPY --from=planner /app/wasm-server-runner-recipe.json wasm-server-runner-recipe.json
#ensure that we are building for nightly wasm32 target by copying the wasm-toolchain
COPY ./wasm/toolchain.toml rust-toolchain.toml

RUN rustup target install x86_64-unknown-linux-musl\
    && apt-get update && apt-get install -y --no-install-recommends \
    musl-tools \
#brotli \
    && rm -rf /var/lib/apt/lists/*

RUN cargo chef cook -p flightless-galaxy --recipe-path recipe.json --target wasm32-unknown-unknown --profile wasm-release
#--bin flightless-galaxy flightless-galaxy --recipe-path recipe.json --target wasm32-unknown-unknown --profile wasm-release
RUN cargo chef cook -p wasm-server-runner --recipe-path recipe.json --target x86_64-unknown-linux-musl --release

# Build application

COPY . .
COPY ./wasm/toolchain.toml rust-toolchain.toml

RUN cargo install --path crates/wasm-server-runner --target x86_64-unknown-linux-musl
RUN cargo build --target wasm32-unknown-unknown --profile wasm-release

# RUN cargo install wasm-pack
# RUN wasm-pack build --target web --out-dir static --profile wasm-release

# RUN wasm-bindgen --out-name wasm_example \
#     --out-dir examples/wasm/target \
#     --target web target/wasm32-unknown-unknown/release/examples/lighting.wasm


#final, size efficient docker layer
FROM alpine:latest AS runtime
# FROM busybox AS runtime
# FROM alpine:latest AS runtime
WORKDIR /app

COPY ./assets .

# RUN apt-get update ; apt-get install -y --no-install-recommends g++ pkg-config libx11-dev libasound2-dev libudev-dev ; rm -rf /var/lib/apt/lists/*
# COPY --from=builder /app/static/ .
COPY --from=builder /app/target/wasm32-unknown-unknown/wasm-release/flightless-galaxy.wasm /app/flightless-galaxy.wasm
COPY --from=builder /usr/local/cargo/bin/wasm-server-runner /

#copy the executable to run the standalone wasm file and runner
# ENTRYPOINT ["/usr/bin/wasm-server-runner", "/app/flightless-galaxy.wasm"]
#"WASM_SERVER_RUNNER_ADDRESS=0.0.0.0:80", 
ENTRYPOINT [ "/wasm-server-runner", "/app/flightless-galaxy.wasm"]
