
#[cfg(target_os = "windows")]
#[cfg(target_arch = "aarch64")]
/// Deno cannot be built for ARM Windows because protobuf does not have any ARM Windows binaries 
pub const V8_SNAPSHOT: &[u8] = &[]; //include_bytes!(env!("SNAPSHOT_PATH"));
