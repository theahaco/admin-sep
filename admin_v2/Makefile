default: build

all: test

test: build
	cargo test

build:
	stellar contract build --out-dir ../target/stellar
	stellar contract optimize --wasm ../target/stellar/example_admin_contract_v2.wasm --wasm-out ../target/stellar/example_admin_contract_v2.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean
