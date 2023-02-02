#!/bin/sh

BUILD_WASMS=${1:-true}
TEST_THREADS=${2:-2}
TESTNAME=$3

if [ $BUILD_WASMS = true ]
then
    ./generate-wasm.sh index_canister_impl
    ./generate-wasm.sh bucket_canister_impl
fi

cargo test --release --package integration_tests $TESTNAME -- --test-threads $TEST_THREADS