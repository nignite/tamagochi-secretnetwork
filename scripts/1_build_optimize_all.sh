#!/bin/bash
set -o errexit -o nounset -o pipefail
command -v shellcheck > /dev/null && shellcheck "$0"


echo "Building  and optimizing contracts"

# iterate over all folders in contracts and packages dir then cnd and run cargo wasm


for DIR in contracts packages; do
    for D in "$DIR"/*; do
        if [ -d "$D" ]; then
            (
                cd "$D"
                echo "Building $D"
                cargo build --release --target wasm32-unknown-unknown
                echo "Building complete"

                echo "Optimzing $D"
                docker run --rm -v "$(pwd)":/contract \
                    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
                    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
                     enigmampc/secret-contract-optimizer  

                echo "Optimizing complete"    
            )
        fi
    done
done

echo "Build and optimization complete. "