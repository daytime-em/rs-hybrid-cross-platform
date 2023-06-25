#!/bin/bash

##################################################
# We call this from an Xcode run script.
##################################################

set -e

export PATH="$HOME/.cargo/bin:$PATH"
readonly RS_MANIFEST_PATH="../rustlib/binding_swift/Cargo.toml"
readonly RS_TARGET_BASE="../../target"

export LIBRARY_PATH="$LIBRARY_PATH:/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/lib"


if [[ $CONFIGURATION == "Release" ]]; then
  echo "BUIlDING FOR RELEASE"
  
  cargo build --release --manifest-path $RS_MANIFEST_PATH --target "aarch64-apple-ios"
  cargo build --release --manifest-path $RS_MANIFEST_PATH --target "aarch64-apple-ios-sim"
  cargo build --release --manifest-path $RS_MANIFEST_PATH --target "x86_64-apple-ios"

  lipo $RS_TARGET_BASE/x86_64-apple-ios/release/libbinding_swift.a \
      $RS_TARGET_BASE/aarch64-apple-ios-sim/release/libbinding_swift.a \
      -create -output $RS_TARGET_BASE/release/libbinding_swift-sim.a

  rm -rf Generated/Rustlib.xcframework
  
  xcodebuild -create-xcframework \
    -library $RS_TARGET_BASE/aarch64-apple-ios/release/libbinding_swift.a \
    -headers Generated/binding_swift/binding_swift.h \
    -library $RS_TARGET_BASE/release/libbinding_swift-sim.a \
    -headers Generated/binding_swift/binding_swift.h \
    -output Generated/Rustlib.xcframework
else
  echo "BUIlDING FOR DEBUG"

  cargo build --manifest-path $RS_MANIFEST_PATH --target "aarch64-apple-ios"
  cargo build --manifest-path $RS_MANIFEST_PATH --target "aarch64-apple-ios-sim"
  cargo build --manifest-path $RS_MANIFEST_PATH --target "x86_64-apple-ios"

  lipo $RS_TARGET_BASE/x86_64-apple-ios/debug/libbinding_swift.a \
      $RS_TARGET_BASE/aarch64-apple-ios-sim/debug/libbinding_swift.a \
      -create -output $RS_TARGET_BASE/debug/libbinding_swift-sim.a

  rm -rf Generated/Rustlib.xcframework

  xcodebuild -create-xcframework \
    -library $RS_TARGET_BASE/aarch64-apple-ios/debug/libbinding_swift.a \
    -headers Generated/binding_swift/binding_swift.h \
    -library $RS_TARGET_BASE/debug/libbinding_swift-sim.a \
    -headers Generated/binding_swift/binding_swift.h \
    -output Generated/Rustlib.xcframework
fi
