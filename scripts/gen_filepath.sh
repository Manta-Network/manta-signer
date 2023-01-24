#!/bin/bash

if [ "$1" == "ubuntu" ]
then
    echo '::set-output name=IMAGE::appimage/MantaSigner_'$3'_amd64.AppImage'
    echo '::set-output name=IMAGE_UPDATE::appimage/MantaSigner_'$3'_amd64.AppImage.tar.gz'
    echo '::set-output name=UPDATE_SIG::appimage/MantaSigner_'$3'_amd64.AppImage.tar.gz.sig'
    echo '::set-output name=RELEASE_IMAGE::MantaSigner-'$2'_'$3'_amd64.AppImage'
    echo '::set-output name=RELEASE_IMAGE_UPDATE::MantaSigner-'$2'_'$3'_amd64.AppImage.tar.gz'
    echo '::set-output name=RELEASE_UPDATE_SIG::MantaSigner-'$2'_'$3'_amd64.AppImage.tar.gz.sig'
elif [ "$1" == "windows" ]
then
    echo '::set-output name=IMAGE::msi/MantaSigner_'$3'_x64_en-US.msi'
    echo '::set-output name=IMAGE_UPDATE::msi/MantaSigner_'$3'_x64_en-US.msi.zip'
    echo '::set-output name=UPDATE_SIG::msi/MantaSigner_'$3'_x64_en-US.msi.zip.sig'
    echo '::set-output name=RELEASE_IMAGE::MantaSigner-'$2'_'$3'_x64.msi'
    echo '::set-output name=RELEASE_IMAGE_UPDATE::MantaSigner-'$2'_'$3'_x64.msi.zip'
    echo '::set-output name=RELEASE_UPDATE_SIG::MantaSigner-'$2'_'$3'_x64.msi.zip.sig'
elif [ "$1" == "macos" ]
then
    echo '::set-output name=IMAGE::dmg/MantaSigner_'$3'_x64.dmg'
    echo '::set-output name=IMAGE_UPDATE::macos/MantaSigner.app.tar.gz'
    echo '::set-output name=UPDATE_SIG::macos/MantaSigner.app.tar.gz.sig'
    echo '::set-output name=RELEASE_IMAGE::MantaSigner-'$2'_'$3'_x64.dmg'
    echo '::set-output name=RELEASE_IMAGE_UPDATE::MantaSigner-'$2'_'$3'.app.tar.gz'
    echo '::set-output name=RELEASE_UPDATE_SIG::MantaSigner-'$2'_'$3'.app.tar.gz.sig'
fi
