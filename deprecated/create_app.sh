    #!/bin/bash

set -e

VERSION=`git describe --tags`
RUST_ARC_OS=x86_64-apple-darwin

# functions
requeststatus() { # $1: requestUUID
    requestUUID=${1?:"need a request UUID"}
    req_status=$(xcrun altool --notarization-info "$requestUUID" \
                              --username "${AC_USERNAME}" \
                              --password "${AC_PASSWORD}" 2>&1 \
                 | awk -F ': ' '/Status:/ { print $2; }' )
    echo "$req_status"
}

notarizefile() {
  filepath=${1:?"need a filepath"}
  identifier=${2:?"need a identifier"}

  # 上传文件
  echo "## uploading $filepath for notarization"
  requestUUID=$(xcrun altool --notarize-app \
                             --primary-bundle-id "$identifier" \
                             --username "${AC_USERNAME}" \
                             --password "${AC_PASSWORD}" \
                             --asc-provider "${AC_PROVIDER}" \
                             --file "$filepath" 2>&1 \
                | awk '/RequestUUID/ { print $NF; }')

  echo "Notarization RequestUUID: $requestUUID"

  if [[ $requestUUID == "" ]]; then
      echo "could not upload for notarization"
      exit 1
  fi

  # wait for status to be not "in progress" any more
  request_status="in progress"
  while [[ "$request_status" == "in progress" ]]; do
      echo -n "waiting... "
      sleep 10
      request_status=$(requeststatus "$requestUUID")
      echo "$request_status"
  done

  # print status information
  xcrun altool --notarization-info "$requestUUID" \
               --username "${AC_USERNAME}" \
               --password "${AC_PASSWORD}"
  echo

  if [[ $request_status != "success" ]]; then
      echo "## could not notarize $filepath"
      exit 1
  fi
}

echo ""
echo "  manta-signer ${VERSION}..."
echo ""
echo -n $VERSION > .version

# need to be tested
rm -rf ./build/bin

sed "s/0.0.0/${VERSION}/" ./build/darwin/Info.plist.src > ./build/darwin/Info.plist
cd ./lib/zkp && cargo build --release --target=${RUST_ARC_OS}
cd ../../
cp ./lib/zkp/target/${RUST_ARC_OS}/release/libzkp.a ./lib/darwin/libzkp.a
CGO_LDFLAGS=-mmacosx-version-min=10.13 ~/go/bin/wails build -package -production -platform darwin -o manta-signer
