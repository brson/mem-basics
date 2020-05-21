#!/bin/bash

set -x -e

cargo install critcmp

cargo bench

critcmp --target-dir=src/memcpy/target new -g ".*/(.*)"
