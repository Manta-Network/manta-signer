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
cp ./sdk/legacy/proving/private-transfer.dat ~/.local/share/manta-signer/transfer_pk.bin
cp ./sdk/legacy/proving/reclaim.dat ~/.local/share/manta-signer/reclaim_pk.bin
