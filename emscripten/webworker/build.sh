#!/bin/bash

set -e

# Note: FORCE_FILESYSTEM is not necessary if C source uses filesystem API

emcc worker.c \
    -s WASM=1 \
    -s "EXPORTED_FUNCTIONS=['_hello']" \
    -s "EXTRA_EXPORTED_RUNTIME_METHODS=['cwrap']" \
    -s 'FORCE_FILESYSTEM=1' \
    -o worker.js \
    --pre-js pre.js \
    --js-library lib.js \
