default: build

all: test

test: build
	cargo test

build:
	stellar scaffold build
	stellar contract optimize --wasm ../target/stellar/example_admin_contract.wasm --wasm-out ../target/stellar/example_admin_contract.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean
