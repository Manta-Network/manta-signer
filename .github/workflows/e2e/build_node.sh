#!/bin/bash
COMMIT_HASH=$1
pwd
cd ~
git clone https://github.com/Manta-Network/Manta.git
cd Manta
pwd
git checkout $COMMIT_HASH
chmod +x scripts/init.sh
./scripts/init.sh
cargo build --release
chmod +x ~/Manta/target/release/manta
