#!/bin/bash
set -ev

echo Installing Protobuf
export PROTOBUF_URL=$1
export PATH=$HOME/.local/protobuf/bin:$PATH
export PROTOC=$HOME/.local/protobuf/bin/protoc

mkdir -p $HOME/.local/protobuf
curl -L -o $HOME/.local/protobuf/protoc.zip $PROTOBUF_URL
unzip $HOME/.local/protobuf/protoc.zip -d $HOME/.local/protobuf
which protoc
protoc --version