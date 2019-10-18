#!/bin/bash -e

# https://www.infradead.org/openconnect/building.html

# Ubuntu 19.10 `sudo apt install openconnect` installs 8.02
# openconnect --version
# OpenConnect version v8.02-1build1
# Using GnuTLS. Features present: TPMv2, PKCS#11, RSA software token, HOTP software token, TOTP software token, Yubikey OATH, System keys, DTLS, ESP
# Supported protocols: anyconnect (default), nc, gp

# https://github.com/openconnect/openconnect
# git clone git@github.com:openconnect/openconnect.git ../openconnect
# cd ../openconnect

# sudo apt install automake libtool gettext libxml2-dev zlib1g-dev
# sudo apt install libproxy-dev libp11-dev libstoken-dev libpcsclite-dev liblz4-dev
# libp11-dev didn't enable PKCS#11 support

./autogen.sh

# --with-vpnc-script=/usr/share/vpnc-scripts/vpnc-script \

./configure \
    --without-gnutls \
    --with-openssl

make clean
make