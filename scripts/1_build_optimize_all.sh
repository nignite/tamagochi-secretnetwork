#!/bin/bash
set -o errexit -o nounset -o pipefail
command -v shellcheck > /dev/null && shellcheck "$0"

echo "Building and optimizing workspace..."
RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown
echo "Optimizations complete. "