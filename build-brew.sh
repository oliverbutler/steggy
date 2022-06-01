#!/bin/bash

# Build with cargo
cargo build --release

# tar the binary
cd target/release
mv main steggy
tar -czvf steggy.tar.gz steggy

# Log the SHA256
echo "SHA256: $(shasum -a 256 steggy.tar.gz)"