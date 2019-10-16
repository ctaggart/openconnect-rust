#!/bin/bash -e

# https://github.com/immunant/c2rust
# git clone git@github.com:immunant/c2rust.git ../c2rust
# cd ../c2rust; cargo build --release

# Make sure to install scan-build which provides the intercept-build executable.
# pip3 install scan-build

cd=$PWD
cd ../openconnect
intercept-build --override-compiler ../openconnect-rust/build-openconnect-osx.sh

cd $cd
# # macOS Catalina 10.15 has that clang version
../c2rust/target/debug/c2rust transpile -o $PWD ../openconnect/compile_commands.json -- \
    -I /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include \
    -I /Library/Developer/CommandLineTools/usr/lib/clang/11.0.0/include

# why are the files being sent to $PWD/openconnect-rust/src ? bug ?
mv openconnect-rust/src src
rm -rf openconnect-rust