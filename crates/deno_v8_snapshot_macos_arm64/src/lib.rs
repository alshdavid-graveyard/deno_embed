#[cfg(target_os = "macos")]
#[cfg(target_arch = "aarch64")]
pub const V8_SNAPSHOT: &[u8] = include_bytes!(env!("SNAPSHOT_PATH"));
