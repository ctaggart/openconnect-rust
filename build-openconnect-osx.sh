#!/bin/bash -e

# https://www.infradead.org/openconnect/building.html
# http://mixablehodgepodge.blogspot.com/2019/01/vpn-compiling-openconnect-for-macos.html

# Install the current openconnect
# to get /usr/local/etc/vpnc-script
# brew install openconnect

# Install the required build tools:
# xcode-select --install
# brew install automake libtool

# Install the required libraries:
# brew install gettext libxml2 libproxy lz4 

# https://github.com/openconnect/openconnect
# git clone git@github.com:openconnect/openconnect.git ../openconnect
# cd ../openconnect
# ../openconnect-rust/build-openconnect-osx.sh

export PKG_CONFIG_PATH="/usr/local/opt/openssl/lib/pkgconfig"
export PATH="/usr/local/opt/gettext/bin:$PATH"
export LDFLAGS="-L/usr/local/opt/gettext/lib"
export CFLAGS="-I/usr/local/opt/gettext/include -I/usr/local/opt/libxml2/include/libxml2"

./autogen.sh

./configure \
    --with-vpnc-script=/usr/local/etc/vpnc-script \
    --without-gnutls \
    --with-openssl

make clean
make