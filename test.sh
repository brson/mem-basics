#!/bin/bash

# Test with miri and asan

set -x -e

export RUSTUP_TOOLCHAIN=nightly-2020-05-15

rustup toolchain install $RUSTUP_TOOLCHAIN
rustup component add miri --toolchain $RUSTUP_TOOLCHAIN
rustup component add rust-src --toolchain $RUSTUP_TOOLCHAIN

cargo install xargo

MIRI_DIR="$(pwd)/target/miri"
ASAN_DIR="$(pwd)/target/asan"

if [[ -z "$SKIP_MIRI" ]]; then
    pushd src/memcpy
    cargo miri test -p memcpy --target-dir="$MIRI_DIR" -- -Zmiri-disable-isolation
    popd
else
    echo "skipping miri"
fi

# TODO test this with asan

export ASAN_OPTIONS="allow_addr2line=true"
export RUSTFLAGS="-Zsanitizer=address"

set +e
uname -a | grep x86_64-unknown-linux-gnu
HAVE_X86=$?
if [[ "$HAVE_X86" -ne 0 ]]; then
    SKIP_ASAN=1
fi
set -e

if [[ -z "$SKIP_ASAN" ]]; then
    cargo test -p memcpy --target-dir="$ASAN_DIR" --target=x86_64-unknown-linux-gnu
else
    echo "skipping asan"
fi
