.PHONY: test

install-wasm-pack:
	 which wasm-pack || curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

build: install-wasm-pack
	wasm-pack build

build-dev:
	wasm-pack build --dev

test: install-wasm-pack
	wasm-pack test --firefox --headless

run-fluvio-websocket-proxy:
	RUST_LOG=debug cargo run --manifest-path ./fluvio-websocket-proxy/Cargo.toml --target $(shell rustup show | grep 'Default host' | sed 's/Default host: //g')

check-fluvio-websocket-proxy:
	RUST_LOG=debug cargo check --manifest-path ./fluvio-websocket-proxy/Cargo.toml --target $(shell rustup show | grep 'Default host' | sed 's/Default host: //g')

webpack-dev:
	npm install
	npm run webpack-dev
