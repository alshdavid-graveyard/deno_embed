#!/bin/bash
set -ev

export SNAPSHOT_NAME="v8-deno-macos-amd64.bin"

uname -a

brew install llvm cmake protobuf make gcc

clang -v

export PATH="/opt/homebrew/opt/llvm/bin:$PATH"

which lld
clang -v

source .github/scripts/utils/install-rust.bash
source .github/scripts/utils/install-protobuf.bash \
  https://github.com/protocolbuffers/protobuf/releases/download/v25.3/protoc-25.3-osx-x86_64.zip

rustup target add x86_64-apple-darwin

cd deno_snapshots
cargo clean
cargo build --release --target x86_64-apple-darwin

cd cli/snapshots/release
XZ_OPT=-9 tar -Jcvf v8-deno-macos-amd64.tar.xz v8-deno-macos-amd64.bin