#!/bin/bash
set -x
set -e
cargo build --release
pushd ./target/release
if test -f "tidii.zip"; then
    rm tidii.zip
fi
codesign -s  "Developer ID Application: David Vernon (3CT7AJ22D9)" --options=runtime tidii
zip tidii.zip tidii
codesign -s  "Developer ID Application: David Vernon (3CT7AJ22D9)" --options=runtime tidii.zip
xcrun notarytool submit tidii.zip  --keychain-profile "rust-notarize-app" --wait
popd
