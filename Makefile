build: 
	@echo "Building NEAR Contracts..."
	@set -e && cd near && RUSTFLAGS='-C link-arg=-s' \
	cargo build --target wasm32-unknown-unknown --release && \
	mkdir -p ./build && cp target/wasm32-unknown-unknown/release/*.wasm \
	./build/contract.wasm
	@echo "Built NEAR Contracts.\n"

	@echo "Building Polygon Contracts..."
	@cd polygon && forge build
	@echo "Built Polygon Contracts.\n"

	@echo "Building Solana Contracts..."
	@cd solana && anchor build && mkdir -p ./build && cp target/deploy/*.so \
	./build/contract.so
	@echo "Built Solana Contracts.\n"

test: 
	@echo "\nTesting NEAR Contracts..."
	@cd near && cargo test -- --nocapture

	@echo "\nTesting Polygon Contracts..."
	@cd polygon && forge test

	@echo "\nTesting Solana Contracts..."
	@cd solana && anchor test