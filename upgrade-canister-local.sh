#!/bin/sh

IDENTITY=$1
CANISTER_TO_UPGRADE=$2
VERSION=$3

# Pass in the dfx identity name
# eg './upgrade-canister-local.sh openstorage index 1.0.0'
./generate-wasm.sh index_canister_impl
./generate-wasm.sh bucket_canister_impl

./compress-wasm.sh bucket_canister_impl

INDEX_CANISTER_ID=$(dfx canister id index)

cargo run \
  --manifest-path backend/canister_upgrader/Cargo.toml \
  'http://127.0.0.1:8000/' \
  $IDENTITY \
  $INDEX_CANISTER_ID \
  $CANISTER_TO_UPGRADE \
  $VERSION \