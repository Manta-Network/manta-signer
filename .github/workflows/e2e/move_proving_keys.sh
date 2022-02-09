#!/bin/bash

mkdir ~/.local
mkdir ~/.local/share
mkdir ~/.local/share/manta-signer

echo sdk
ls ./sdk
echo legacy
ls ./sdk/legacy
echo proving
ls ./sdk/legacy/proving

cp ./sdk/legacy/proving/private-transfer.dat ~/.local/share/manta-signer
cp ./sdk/legacy/proving/reclaim.dat ~/.local/share/manta-signer
