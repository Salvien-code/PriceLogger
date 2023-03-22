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
	@cd solana && anchor build
	@echo "Built Solana Contracts.\n"

test: 
	@echo "Testing NEAR Contracts..."
	@cd near && cargo test -- --nocapture

	@echo "Testing Polygon Contracts..."
	@cd polygon && forge test

	@echo "Testing Solana Contracts..."
	@cd solana && anchor test