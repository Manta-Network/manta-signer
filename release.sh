#!/usr/bin/env sh

# Prompts the user to check if they performed a pre-release task. If the user denies
# having completed the task, the program exits.
prompt() {
    echo "$1 (y/N) "
    read answer
    case $answer in
        y|Y|yes|Yes|YES) echo "" ;;
        *) exit; ;;
    esac
}

cat << EOF
Manta Signer Release
====================

To build a release, follow the prompts below.

EOF

cat << EOF
VERSION NUMBERS
===============

The version numbers in the following files must be updated:
  1. Cargo.toml
  2. ui/src-tauri/Cargo.toml
  3. ui/src-tauri/tauri.conf.json
  4. ui/public/about.html
  5. ui/package.json

EOF
prompt "Did you update the version numbers?"

echo "release"

