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
                cargo wasm 
                echo "Building complete"

                echo "Optimzing $D"
                docker run -it --rm \
                     -p 26657:26657 -p 26656:26656 -p 1337:1337 \
                     -v $(pwd):/root/code \
                     --name secretdev enigmampc/secret-network-sw-dev
                echo "Optimizing complete"    
            )
        fi
    done
done

echo "Build and optimization complete. "