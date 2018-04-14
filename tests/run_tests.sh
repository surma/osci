#!/bin/bash

set -e

. $NVM_DIR/nvm.sh
cargo test
cargo build --release --example osciwasm --target wasm32-unknown-unknown
cd tools/osciwasm
npm test
