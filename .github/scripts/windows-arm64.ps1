$env:SNAPSHOT_NAME = "v8-deno-windows-arm64.bin"

echo "skipping ARM for now"
exit 0

cd deno_snapshots

rustup target add aarch64-pc-windows-msvc
cargo build --release --target aarch64-pc-windows-msvc

cd snapshots/release
tar -czvf v8-deno-windows-arm64.tar.xz v8-deno-windows-arm64.bin
