#!/bin/bash
# COMMIT_HASH=$1
# cd ~
# git clone https://github.com/Manta-Network/Manta.git
# cd Manta
# git checkout $COMMIT_HASH
# chmod +x scripts/init.sh
# ./scripts/init.sh
# cargo build --release
# chmod +x ~/Manta/target/release/manta
mkdir Manta
cd Manta
curl https://manta-ops.s3.amazonaws.com/Dolphin-v0.2.1-841eb2f-x86_64-linux-gnu/manta --output dolphin
chmod +x dolphin
