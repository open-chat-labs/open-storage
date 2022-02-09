#!/bin/sh

IDENTITY=$1

# Pass in the dfx identity name
# eg './deploy-prod-test openstorage'
./generate-wasm.sh index_canister_impl
./generate-wasm.sh bucket_canister_impl

./compress-wasm.sh bucket_canister_impl

INDEX_CANISTER_ID=$(dfx canister --network ic_test --no-wallet id index)

cargo run \
  --manifest-path backend/canister_installer/Cargo.toml \
  'https://ic0.app/' \
  true \
  $IDENTITY \
  $INDEX_CANISTER_ID \