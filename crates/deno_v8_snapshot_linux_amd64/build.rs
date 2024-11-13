use std::path::PathBuf;

use walkdir::WalkDir;

fn main() {
    let profile = std::env::var("PROFILE").unwrap();
    let target = std::env::var("TARGET").unwrap();
    let out_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let crate_path = PathBuf::from(&out_dir);
    let workspace_path = crate_path.parent().unwrap().parent().unwrap();
    let deno_target_path = {
      let target_path = workspace_path.join("deno").join("target");
      if target_path.join("release").exists() {
        target_path
      } else {
        target_path.join(target)
      }
    };
    
    let deno_output_path = if profile == "release" {
      deno_target_path.join("release")
    } else {
      deno_target_path.join("debug")
    };

    if profile == "debug" && !deno_output_path.exists() {
      // Provide mock file for rust-analyzer
      println!("cargo:rustc-env=SNAPSHOT_PATH={}", crate_path.join("Cargo.toml").to_str().unwrap());
      return
    }

    for entry in WalkDir::new(deno_output_path) {
      let entry = entry.unwrap().path().to_path_buf();
      let entry_str = entry.to_str().unwrap();
      if entry_str.ends_with("CLI_SNAPSHOT.bin") {
        println!("cargo:rustc-env=SNAPSHOT_PATH={}", entry_str);
        return
      }
    }    

    panic!("Unable to find SNAPSHOT.bin in Deno")
}
