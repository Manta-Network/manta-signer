#!/bin/bash

function kill_it_with_fire() {
    kill -9 $(ps -ef | grep manta-front-end | awk '{ print $2 }')
    echo "All done!"
}

# TODO: set up local testnet..
# cd ../manta-pc-launch
# yarn start dolphin-local.json

# TODO: patch up the front-end with the sdk?!?!
# In a separate process....
cd ../manta-front-end
yarn && yarn build
yarn serve -l 3000 -s build &
SERVE_PID=$!
echo "Serving UI on ${SERVE_PID}"

cd -

yarn

node index.js


kill_it_with_fire
