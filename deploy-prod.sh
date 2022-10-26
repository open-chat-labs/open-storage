#!/bin/sh

IDENTITY=$1
TEST_MODE=$2

# Pass in the dfx identity name
# eg './deploy-local openstorage'
./generate-wasm.sh index_canister_impl
./generate-wasm.sh bucket_canister_impl

INDEX_CANISTER_ID=$(dfx canister --network ic id index)

cargo run \
  --manifest-path backend/canister_installer/Cargo.toml \
  'https://ic0.app/' \
  $TEST_MODE \
  $IDENTITY \
  $INDEX_CANISTER_ID \