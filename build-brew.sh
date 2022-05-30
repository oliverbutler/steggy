#!/bin/bash

# Build with cargo
cargo build --release

# tar the binary
cd target/release
mv main imgfu
tar -czvf imgfu.tar.gz imgfu

# Log the SHA256
echo "SHA256: $(shasum imgfu.tar.gz)"