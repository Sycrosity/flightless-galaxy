run:
	cargo run --profile dev

build:
	cargo build --release

install-wasm-prereqs:
	# cargo install wasm-bindgen-cli
	cargo install wasm-server-runner

install-wasm: install-wasm-prereqs
	rustup target install wasm32-unknown-unknown

run-wasm: install-wasm
	# Run a minimal server with the game compiled into WASM
	cargo run --target wasm32-unknown-unknown

run-wasm-release: install-wasm
	# Run a minimal server with the game compiled into WASM
	cargo run --target wasm32-unknown-unknown --profile wasm-release

watch-wasm:
	cargo watch -i wasm -x "run --target wasm32-unknown-unknown --profile wasm-dev"

build-wasm: install-wasm
	cargo build --target wasm32-unknown-unknown --profile wasm-release

docker:
	docker-compose down --remove-orphans
	docker-compose up --build
