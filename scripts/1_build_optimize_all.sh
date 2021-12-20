#!/bin/bash
set -o errexit -o nounset -o pipefail
command -v shellcheck > /dev/null && shellcheck "$0"


rm -rf artifacts; mkdir artifacts
echo "Building  and optimizing contracts"


for DIR in contracts packages; do
    for D in "$DIR"/*; do
        if [ -d "$D" ]; then
            (
                cd "$D"
                echo "Building $D"
                echo "Building complete"

                echo "Optimzing $D"
                docker run --rm -v "$(pwd)":/contract \
                    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
                    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
                     enigmampc/secret-contract-optimizer:1.0.5

                echo "Optimizing complete"    
                echo "Copying artifacts"
                name = $($D | tr "/")
                cargo build --release --target wasm32-unknown-unknown
                cp *.wasm.gz artifacts/"$name".wasm.gz
                echo "Copying complete"
            )
        fi
    done
done

echo "Build and optimization complete. "