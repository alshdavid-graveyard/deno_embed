use std::fs;
use std::path::PathBuf;

use walkdir::WalkDir;

fn main() {
    let profile = std::env::var("PROFILE").unwrap();
    let target = std::env::var("TARGET").unwrap();
    let crate_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let crate_path = PathBuf::from(&crate_path);
    let workspace_path = crate_path.parent().unwrap().parent().unwrap();

    let deno_target_path = {
      let target_path = workspace_path.join("deno").join("target");
      if target_path.join(&target).exists() {
        target_path.join(target)
      } else {
        target_path
      }
    };
    
    let deno_output_path = if profile == "release" {
      deno_target_path.join("release")
    } else {
      deno_target_path.join("debug")
    };

    // panic!("{}", deno_output_path.display());

    if profile == "debug" && !deno_output_path.exists() {
      // Provide mock file for rust-analyzer
      println!("cargo:rustc-env=SNAPSHOT_PATH={}", crate_path.join("Cargo.toml").to_str().unwrap());
      return
    }

    for entry in WalkDir::new(deno_output_path) {
      let entry = entry.unwrap().path().to_path_buf();
      let entry_str = entry.to_str().unwrap();
      if entry_str.ends_with("CLI_SNAPSHOT.bin") {
        fs::copy(entry_str, crate_path.join("src").join("snapshot.bin")).unwrap();
        println!("cargo:rustc-env=SNAPSHOT_PATH=snapshot.bin");
        return
      }
    }    

    panic!("Unable to find SNAPSHOT.bin in Deno")
}
