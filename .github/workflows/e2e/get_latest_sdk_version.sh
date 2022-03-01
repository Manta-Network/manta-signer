#!/bin/bash
BRANCH=$1

git ls-remote https://github.com/manta-network/sdk.git $BRANCH | awk '{ print $1}'
