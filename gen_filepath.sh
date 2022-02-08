#!/bin/bash

if [ "$1" == "ubuntu" ]
then
    echo '::set-output name=IMAGE::appimage/manta-signer_'$3'_amd64.AppImage'
    echo '::set-output name=IMAGE_UPDATE::appimage/manta-signer_'$3'_amd64.AppImage.tar.gz'
    echo '::set-output name=UPDATE_SIG::appimage/manta-signer_'$3'_amd64.AppImage.tar.gz.sig'
    echo '::set-output name=RELEASE_IMAGE::manta-signer-'$2'_'$3'_amd64.AppImage'
    echo '::set-output name=RELEASE_IMAGE_UPDATE::manta-signer-'$2'_'$3'_amd64.AppImage.tar.gz'
    echo '::set-output name=RELEASE_UPDATE_SIG::manta-signer-'$2'_'$3'_amd64.AppImage.tar.gz.sig'
elif [ "$1" == "windows" ]
then
    echo '::set-output name=IMAGE::msi/manta-signer_'$3'_x64.msi'
    echo '::set-output name=IMAGE_UPDATE::msi/manta-signer_'$3'_x64.msi.zip'
    echo '::set-output name=UPDATE_SIG::msi/manta-signer_'$3'_x64.msi.zip.sig'
    echo '::set-output name=RELEASE_IMAGE::manta-signer-'$2'_'$3'_x64.msi'
    echo '::set-output name=RELEASE_IMAGE_UPDATE::manta-signer-'$2'_'$3'_x64.msi.zip'
    echo '::set-output name=RELEASE_UPDATE_SIG::manta-signer-'$2'_'$3'_x64.msi.zip.sig'
elif [ "$1" == "macos" ]
then
    echo '::set-output name=IMAGE::dmg/manta-signer_'$3'_x64.dmg'
    echo '::set-output name=IMAGE_UPDATE::macos/manta-signer.app.tar.gz'
    echo '::set-output name=UPDATE_SIG::macos/manta-signer.app.tar.gz.sig'
    echo '::set-output name=RELEASE_IMAGE::manta-signer-$2_$3_x64.dmg'
    echo '::set-output name=RELEASE_IMAGE_UPDATE::manta-signer-'$2'_'$3'.app.tar.gz'
    echo '::set-output name=RELEASE_UPDATE_SIG::manta-signer-'$2'_'$3'.app.tar.gz.sig'
fi
