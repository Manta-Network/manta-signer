#!/bin/bash
COMMIT_HASH=$1

cd ~
git clone https://github.com/Manta-Network/manta-signer.git
cd manta-signer/examples
git checkout $COMMIT_HASH
