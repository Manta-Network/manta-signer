#!/bin/bash

# Create 10-round simulation instructions and save output
cd js/e2e/simulation
cargo run 10 | tail -n +2 > out.json
cat out.json

# Run 3 headless copies of signer
cd ../../..
./target/release/examples/test_server 127.0.0.1:29988 &
./target/release/examples/test_server 127.0.0.1:29989 &
./target/release/examples/test_server 127.0.0.1:29990 &

# Run node
~/Manta/manta             \
    --chain dev           \
    --ws-port 9944        \
    --port 30333          \
    --alice               \
    --tmp                 \
    --rpc-cors all        \
    --unsafe-ws-external  \
    --unsafe-rpc-external \
    --rpc-methods=Unsafe  &

# Run test
cd js
yarn test
