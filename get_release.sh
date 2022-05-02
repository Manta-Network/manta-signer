#!/bin/bash

DOWNLOAD_URL=$(curl https://api.github.com/repos/Manta-Network/Manta/releases | jq -r '.[] | select(.tag_name == "v3.1.5") | .assets[] | select(.name == "manta") | .browser_download_url')

echo "URL: ${DOWNLOAD_URL}"

curl -OL ${DOWNLOAD_URL}
chmod +x manta
