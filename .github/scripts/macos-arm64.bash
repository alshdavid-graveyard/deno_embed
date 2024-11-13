#!/bin/bash
set -ev

export SNAPSHOT_NAME="v8-deno-macos-arm64.bin"

uname -a

brew install llvm cmake protobuf make gcc

clang -v

export PATH="/opt/homebrew/opt/llvm/bin:$PATH"

which lld
clang -v

source .github/scripts/utils/install-rust.bash
source .github/scripts/utils/install-protobuf.bash \
  https://github.com/protocolbuffers/protobuf/releases/download/v25.3/protoc-25.3-osx-aarch_64.zip

rustup target add aarch64-apple-darwin

cd deno
cargo build --release --target aarch64-apple-darwin

cd ../
cargo build --release