#!/bin/bash

GITHUB_URL=https://github.com/TaceoLabs/co-snarks/raw/refs/heads/main/co-noir/co-noir/examples/data/

DIR=./config
mkdir -p $DIR

curl -L -o $DIR/cert0.der $GITHUB_URL/cert0.der
curl -L -o $DIR/cert1.der $GITHUB_URL/cert1.der
curl -L -o $DIR/cert2.der $GITHUB_URL/cert2.der
curl -L -o $DIR/key0.der $GITHUB_URL/key0.der
curl -L -o $DIR/key1.der $GITHUB_URL/key1.der
curl -L -o $DIR/key2.der $GITHUB_URL/key2.der


GITHUB_URL=https://github.com/TaceoLabs/co-snarks/raw/refs/heads/main/co-noir/co-noir/examples/test_vectors/

curl -L -o $DIR/bn254_g1.dat $GITHUB_URL/bn254_g1.dat
curl -L -o $DIR/bn254_g2.dat $GITHUB_URL/bn254_g2.dat