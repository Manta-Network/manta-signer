#!/bin/bash

#DRIVER_VERSION=$(google-chrome --version | awk '{print $3}')
DRIVER_VERSION=101.0.4951.41
FILE_NAME=chromedriver_linux64.zip
CHROME_DRIVER=https://chromedriver.storage.googleapis.com/${DRIVER_VERSION}/${FILE_NAME}

curl -L ${CHROME_DRIVER} --output ${FILE_NAME}

unzip ${FILE_NAME}

chmod +x chromedriver
