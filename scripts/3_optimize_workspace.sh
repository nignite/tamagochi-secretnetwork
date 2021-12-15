#!/bin/bash
set -o errexit -o nounset -o pipefail
command -v shellcheck > /dev/null && shellcheck "$0"
# See https://www.shellcheck.net/wiki/SC2187

export PATH="$PATH:/root/.cargo/bin"

# Suffix for non-Intel built artifacts
MACHINE=$(uname -m)
SUFFIX=${MACHINE#x86_64}
SUFFIX=${SUFFIX:+-$SUFFIX}

rustup toolchain list
cargo --version

rm -rf artifacts
mkdir -p artifacts

echo "Building contracts"

RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown --locked

echo "Optimizing artifacts in workspace ..."

TMPARTIFACTS=$(mktemp -p "$(pwd)" -d artifacts.XXXXXX)
# Optimize artifacts
(
  cd "$TMPARTIFACTS"

  for WASM in ../target/wasm32-unknown-unknown/release/*.wasm; do
    NAME=$(basename "$WASM" .wasm)${SUFFIX}.wasm
    echo "Optimizing $NAME ..."
    wasm-opt -Oz "$WASM" -o "$NAME"
  done
  echo "Moving wasm files ..."
  mv ./*.wasm ../artifacts
)
rm -rf "$TMPARTIFACTS"
echo "Post-processing artifacts in workspace ..."
(
  cd artifacts
  sha256sum -- *.wasm | tee checksums.txt
)

echo "done"
