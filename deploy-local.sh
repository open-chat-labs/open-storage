#!/bin/sh

IDENTITY=$1
TEST_MODE=true

# Pass in the dfx identity name
# eg './deploy-local openstorage'
./generate-wasm.sh index_canister_impl
./generate-wasm.sh bucket_canister_impl

./compress-wasm.sh bucket_canister_impl

dfx --identity $IDENTITY canister create index

INDEX_CANISTER_ID=$(dfx canister id index)

cargo run \
  --manifest-path backend/canister_installer/Cargo.toml \
  'http://127.0.0.1:8000/' \
  $TEST_MODE \
  $IDENTITY \
  $INDEX_CANISTER_ID \
