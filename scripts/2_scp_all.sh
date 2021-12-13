#!/bin/bash
set -o errexit -o nounset -o pipefail
command -v shellcheck > /dev/null && shellcheck "$0"

# sends all .wasm binaries to the host machine, later to be uploaded on the blockchain
# NOT REQUIRED, SPECIFIC USAGE


REMOTE_ADRESS=192.168.0.200
HOST=cosmos

for contract in ./*/; do
    echo "Building all contracts..."
    (
        cd $contract
        scp target/wasm32-unknown-unknown/release/*.wasm $HOST@$REMOTE_ADRESS:~/
    )
done