#!/bin/bash
set -o errexit -o nounset -o pipefail
command -v shellcheck > /dev/null && shellcheck "$0"

for contract in ./*/; do
    echo "Building all contracts..."
    (
        cd $contract
        cargo build --locked
        cargo wasm --locked
        cargo schema --locked
    )
done
echo "Building contracts complete"