#!/bin/bash -e
# Make sure to install scan-build which provides the intercept-build executable.
# pip3 install scan-build
cd=$PWD
cd ../openconnect
intercept-build --override-compiler ../openconnect-rust/build-openconnect-osx.sh
cd $cd