#!/bin/bash

set -x -e

cargo install critcmp

export RUSTFLAGS=-Ctarget-cpu=native

cargo bench

critcmp --target-dir=src/memcpy/target new -g ".*/(.*)"
