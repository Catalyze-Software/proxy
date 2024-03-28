# Build script proxy canister

# Generate candid
cargo test candid -p canister

# Build wasm
cargo build -p canister --release --target wasm32-unknown-unknown

# ic-wasm shrink
ic-wasm target/wasm32-unknown-unknown/release/canister.wasm -o target/wasm32-unknown-unknown/release/canister.wasm shrink

# ic-wasm optimize
ic-wasm target/wasm32-unknown-unknown/release/canister.wasm -o target/wasm32-unknown-unknown/release/canister.wasm optimize O3

# Gzip wasm
gzip -c target/wasm32-unknown-unknown/release/canister.wasm > wasm/canister.wasm.gz
