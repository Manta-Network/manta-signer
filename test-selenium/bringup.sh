#!/bin/bash

function cleanup () {
    kill -9 $(ps -ef | grep manta-pc-launch | awk '{print $2}')
    kill -9 $(ps -ef | grep e2e-local | awk '{print $2}')
    kill -9 $(ps -ef | grep test_server | awk '{print $2}')
    kill -9 $(ps -ef | grep 'yarn serve' | awk '{print $2}')
    cd ${FRONTEND_HOME}; rm -rf build*
}

function wait_for () {
    echo "Waiting to see $2"
    tail -f $1 | sed "/$2/ q"
    echo "Good to go!"
}

trap cleanup SIGINT SIGKILL EXIT

SIGNER_HOME=${SIGNER_HOME:-$PWD}
PC_LAUNCH_HOME=${PC_LAUNCH_HOME:-./manta-pc-launch}
FRONTEND_HOME=${FRONTEND_HOME:-./manta-front-end}

cd ${PC_LAUNCH_HOME}
yarn && yarn start e2e-local.json > pc_launch_output &

wait_for pc_launch_output 'LAUNCH COMPLETE'

echo "Manta devnet is up!"

cd -
cd ${FRONTEND_HOME}
yarn --production=false
yarn build

# somehow yarn does not forward env vars.....
cp -r build build2
cp -r build build3

sed -i 's/29987/29988/g' $(grep -rl 29987 build)
sed -i 's/29987/29989/g' $(grep -rl 29987 build2)
sed -i 's/29987/29990/g' $(grep -rl 29987 build3)

yarn serve -S build -l 3000 &
yarn serve -S build2 -l 3001 &
yarn serve -S build3 -l 3002 &

echo "Serving frontend."

echo "Moving to ${SIGNER_HOME}"

cd ${SIGNER_HOME}
cd js/e2e/simulation
cargo run 10 | tail -n +2 > out.json
cp out.json ${SIGNER_HOME}/test-selenium

echo "We live in a simulation:"

cd -
cargo b --release --features=unsafe-disable-cors --example test_server

target/release/examples/test_server 127.0.0.1:29988 > alice_signer &
wait_for alice_signer 'serving signer API'

target/release/examples/test_server 127.0.0.1:29989 > bob_signer &
wait_for bob_signer 'serving signer API'

target/release/examples/test_server 127.0.0.1:29990 > charlie_signer &
wait_for charlie_signer 'serving signer API'

echo "Signers are up!"
# 6. mint some coins

wait
echo "OK, cleaning up"
