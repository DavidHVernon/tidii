#!/bin/bash
set -x
cd /usr/local/bin
if test -f "tidii.zip"; then
    rm tidii.zip
fi
curl https://raw.githubusercontent.com/DavidHVernon/tidii/master/release/0.1.0/tidii.zip --output tidii.zip
if test -f "tidii"; then
    rm tidii
fi
unzip tidii.zip
rm tidii.zip 
