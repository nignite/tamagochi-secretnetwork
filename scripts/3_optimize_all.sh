#!/bin/bash
set -o errexit -o nounset -o pipefail
command -v shellcheck > /dev/null && shellcheck "$0"

for contract in ./contracts/*/; do
    echo "Optimizing all contracts..."
    (
        cd $contract
        docker run --rm -v "$(pwd)":/contract \
        --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
        --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
        enigmampc/secret-contract-optimizer  
    )
done

for token in ./packages/*/; do
    echo "Optimizing all contracts..."
    (
        cd $token
        docker run --rm -v "$(pwd)":/contract \
        --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
        --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
        enigmampc/secret-contract-optimizer  
    )
done