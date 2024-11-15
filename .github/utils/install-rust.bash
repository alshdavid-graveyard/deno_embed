#!/bin/bash
set -ev

echo Installing Rust
export RUSTUP_HOME=$HOME/.local/rust/rustup
export CARGO_HOME=$HOME/.local/rust/cargo
export PATH=$HOME/.local/rust/cargo/bin:$PATH

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --no-modify-path -y
which cargo