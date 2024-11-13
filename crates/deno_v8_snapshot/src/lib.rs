#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub use deno_v8_snapshot_macos_arm64::V8_SNAPSHOT;

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub use deno_v8_snapshot_macos_amd64::V8_SNAPSHOT;

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
pub use deno_v8_snapshot_linux_arm64::V8_SNAPSHOT;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub use deno_v8_snapshot_linux_amd64::V8_SNAPSHOT;

#[cfg(all(target_os = "windows", target_arch = "aarch64"))]
pub use deno_v8_snapshot_windows_arm64::V8_SNAPSHOT;

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub use deno_v8_snapshot_windows_amd64::V8_SNAPSHOT;

