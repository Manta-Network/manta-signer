#!/bin/bash

cd ~
mkdir Manta
cd Manta
curl https://manta-ops.s3.amazonaws.com/Dolphin-v0.2.1-841eb2f-x86_64-linux-gnu/manta --output manta
chmod +x manta
