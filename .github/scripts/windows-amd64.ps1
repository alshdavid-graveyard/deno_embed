$CWD = (Get-Location).Path
$env:SNAPSHOT_NAME = "v8-deno-windows-amd64.bin"

choco install devbox-unzip

echo "Installing Protobuf"
$PROTOC_URL = "https://github.com/protocolbuffers/protobuf/releases/download/v25.3/protoc-25.3-win64.zip"
$PROTOC_DIR = "$HOME\local\protoc"
New-Item -Path "$PROTOC_DIR" -ItemType Directory  -Force
curl -L -o "$PROTOC_DIR\protoc.zip" $PROTOC_URL
unzip "$PROTOC_DIR\protoc.zip" -d $PROTOC_DIR
$env:Path += ";$PROTOC_DIR\bin"
protoc --version

rustup target add x86_64-pc-windows-msvc

cd deno_snapshots
cargo build --release --target x86_64-pc-windows-msvc

cd cli/snapshots/release
tar -czvf v8-deno-windows-amd64.tar.xz v8-deno-windows-amd64.bin
