#!/bin/bash

# Create 10-round simulation instructions and save output
cd js/e2e/simulation
cargo run 10 | tail -n +2 > out.json
cat out.json

# Run 3 headless copies of signer
cd ../../..
cargo run --example test_server --release --features=unsafe-disable-cors -- http://127.0.0.1:29988 &
cargo run --example test_server --release --features=unsafe-disable-cors -- http://127.0.0.1:29989 &
cargo run --example test_server --release --features=unsafe-disable-cors -- http://127.0.0.1:29990 &

# Run node
~/Manta/target/release/manta  --chain dev --ws-port 9944 --port 30333 --alice \
--tmp --rpc-cors all --unsafe-ws-external --unsafe-rpc-external \
--rpc-methods=Unsafe &

# Run test
cd js
yarn test
