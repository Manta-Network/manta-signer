#!/bin/bash

echo $BRANCH $TARGET $VERSION $SUFFIX

if [[ "$BRANCH" == "master" ]] && [[ "$TARGET" == "win64" ]]
then
    echo "::set-output name=SUFFIXED::$VERSION"
elif [[ "$BRANCH" == "master" ]]
then
    echo "::set-output name=SUFFIXED::$VERSION-$SUFFIX"
elif [[ "$TARGET" == "win64" ]]
then
    echo "::set-output name=SUFFIXED::$VERSION"
else
    echo "::set-output name=SUFFIXED::$VERSION-$((100 + $SUFFIX))"
fi
