#!/bin/bash

mkdir ~/.local
mkdir ~/.local/share
mkdir ~/.local/share/manta-signer
mv ./sdk/zkp/reclaim_pk.bin ~/.local/share/manta-signer
mv ./sdk/zkp/transfer_pk.bin ~/.local/share/manta-signer
ls ~/.local/share/manta-signer
