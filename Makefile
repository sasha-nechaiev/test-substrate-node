run-tmp:
	SKIP_WASM= cargo run -- --dev --tmp -lruntime=debug

test:
	SKIP_WASM= cargo test --all

build:
	SKIP_WASM= cargo build

check:
	SKIP_WASM= cargo check