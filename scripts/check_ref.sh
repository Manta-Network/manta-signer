#!/bin/bash

echo $BRANCH $TARGET $VERSION $SUFFIX

if [[ "$BRANCH" == "master" ]]
then
    echo "::set-output name=SUFFIXED::$VERSION-$SUFFIX"
else
    echo "::set-output name=SUFFIXED::$VERSION-$((100 + $SUFFIX))"
fi
