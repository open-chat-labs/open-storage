#!/bin/bash
echo "1. cargo build"
cargo build

echo "2. cargo clippy"
cargo clippy

echo "3. cargo fmt"
cargo fmt

echo "4. cargo test"
cargo test --lib

echo "5. validate candid syntax"
./validate-candid-syntax.sh

echo "6. validate candid matches rust"
./validate-candid-matches-rust.sh