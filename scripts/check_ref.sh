#!/bin/bash

echo $BRANCH $TARGET $VERSION $SUFFIX

if [[ "$BRANCH" == "master" ]] && [[ "$TARGET" == "win64" ]]
then
    echo "::set-output name=SUFFIXED::$VERSION.$SUFFIX"
elif [[ "$BRANCH" == "master" ]]
if [[ "$BRANCH" == "master" ]]
then
    echo "::set-output name=SUFFIXED::$VERSION-$SUFFIX"
elif [[ "$TARGET" == "win64" ]]
then
   echo "::set-output name=SUFFIXED::$VERSION.$((100 + $SUFFIX))"
else
    echo "::set-output name=SUFFIXED::$VERSION-$((100 + $SUFFIX))"
fi
