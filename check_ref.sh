#!/bin/bash

if [[ "x$1" == "xmaster" ]]
then
    echo '::set-output name=BETA_PREFIX::'
else
    echo '::set-output name=BETA_PREFIX::beta'
fi
