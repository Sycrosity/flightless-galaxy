run:
	cargo run

build:
	cargo build

install-wasm-prereqs:
	cargo install wasm-bindgen-cli
# cargo install wasm-server-runner

install-wasm: install-wasm-prereqs
	rustup target install wasm32-unknown-unknown

run-wasm: install-wasm
	# Run a minimal server with the game compiled into WASM
	cargo run --target wasm32-unknown-unknown

watch-wasm:
	cargo watch -i dist -x "build --target wasm32-unknown-unknown" -s "wasm-bindgen --out-dir ./dist/ --target web ./target/wasm32-unknown-unknown/debug/flightless-galaxy.wasm"

build-wasm: install-wasm
	cargo build --target wasm32-unknown-unknown
	wasm-bindgen --out-dir ./dist/ --target web ./target/wasm32-unknown-unknown/debug/flightless-galaxy.wasm 

docker:
	docker-compose down --remove-orphans
	docker-compose up --build