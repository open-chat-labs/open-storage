#!/bin/sh

IDENTITY=$1
CANISTER_NAME=$2
VERSION=$3

# Pass in the dfx identity name
# eg './upgrade-canister-prod.sh openstorage index 1.0.0'
./generate-wasm.sh ${CANISTER_NAME}_canister_impl

INDEX_CANISTER_ID=$(dfx canister --network ic id index)

cargo run \
  --manifest-path backend/canister_upgrader/Cargo.toml \
  'https://ic0.app/' \
  $IDENTITY \
  $INDEX_CANISTER_ID \
  $CANISTER_NAME \
  $VERSION \

TAG=v$VERSION-$CANISTER_NAME

git tag $TAG HEAD
git push origin tag $TAG