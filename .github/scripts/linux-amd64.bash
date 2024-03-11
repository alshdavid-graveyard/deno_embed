#!/bin/bash
set -ev

export SNAPSHOT_NAME="v8-deno-linux-amd64.bin"

uname -a

sudo apt-get update

source .github/scripts/utils/install-rust.bash
source .github/scripts/utils/install-protobuf.bash \
  https://github.com/protocolbuffers/protobuf/releases/download/v25.3/protoc-25.3-linux-x86_64.zip

cd deno_snapshots
cargo build --release

cd cli/snapshots/release
XZ_OPT=-9 tar -Jcvf v8-deno-linux-amd64.tar.xz v8-deno-linux-amd64.bin