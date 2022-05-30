#!/bin/bash

# Build with cargo
cargo build --release

# tar the binary
cd target/release
mv main steg
tar -czvf steg.tar.gz steg

# Log the SHA256
echo "SHA256: $(shasum -a 256 steg.tar.gz)"