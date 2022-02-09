#!/bin/bash

mkdir ~/.local
mkdir ~/.local/share
mkdir ~/.local/share/manta-signer

echo current
pwd
ls
echo parent
cd ..
pwd
ls
cp ./sdk/zkp/reclaim_pk.bin ~/.local/share/manta-signer
cp ./sdk/zkp/transfer_pk.bin ~/.local/share/manta-signer
