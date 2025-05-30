#!/usr/bin/env bash

rm -rf data
mkdir -p data

[[ -f "data/key0.der" ]] || cargo run --bin gen_cert -- -k data/key0.der -c data/cert0.der -s localhost -s ip6-localhost -s 127.0.0.1 -s party0
[[ -f "data/key1.der" ]] || cargo run --bin gen_cert -- -k data/key1.der -c data/cert1.der -s localhost -s ip6-localhost -s 127.0.0.1 -s party1
[[ -f "data/key2.der" ]] || cargo run --bin gen_cert -- -k data/key2.der -c data/cert2.der -s localhost -s ip6-localhost -s 127.0.0.1 -s party2

cargo run -- --config-file config1.toml &
cargo run -- --config-file config2.toml &
cargo run -- --config-file config3.toml 
