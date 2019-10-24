#!/bin/bash -e

# https://github.com/immunant/c2rust
# git clone git@github.com:immunant/c2rust.git ../c2rust
# cd ../c2rust; cargo build --release

# # macOS Catalina 10.15 has that clang version
RUST_BACKTRACE=1 ../c2rust/target/release/c2rust transpile \
    -o $PWD \
    --overwrite-existing \
    --emit-build-files \
    --reorganize-definitions \
    ../openconnect/compile_commands.json \
    -- \
    -I /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include \
    -I /Library/Developer/CommandLineTools/usr/lib/clang/11.0.0/include \
    -fno-blocks \
    -iframework /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/System/Library/Frameworks