use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
  let out_dir = env::var("OUT_DIR").unwrap();
  let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
  let profile = env::var("PROFILE").unwrap();

  let zig_target = rust_target_to_zig();
  let optimize = if profile == "release" {
    "ReleaseFast"
  } else {
    "Debug"
  };

  build_zig(&manifest_dir, &out_dir, &zig_target, optimize);
  generate_bindings(&manifest_dir, &out_dir);

  let lib_dir = Path::new(&out_dir).join("zig-out").join("lib");
  println!("cargo:rustc-link-search=native={}", lib_dir.display());
  println!("cargo:rustc-link-lib=static=opentui");

  if cfg!(target_os = "macos") {
    println!("cargo:rustc-link-lib=c");
  } else if cfg!(target_os = "linux") {
    println!("cargo:rustc-link-lib=c");
    println!("cargo:rustc-link-lib=m");
  }

  println!("cargo:rerun-if-changed=opentui.h");
  println!("cargo:rerun-if-changed=build.zig");
  println!("cargo:rerun-if-changed=build.zig.zon");
  println!(
    "cargo:rerun-if-changed={}",
    manifest_dir
      .join("vendor/opentui/packages/core/src/zig")
      .display()
  );
}

fn build_zig(manifest_dir: &Path, out_dir: &str, zig_target: &str, optimize: &str) {
  let mut cmd = Command::new("zig");
  cmd
    .arg("build")
    .arg(format!("-Doptimize={optimize}"))
    .arg(format!("-Dtarget={zig_target}"))
    .arg("--prefix")
    .arg(format!("{out_dir}/zig-out"))
    .arg("--cache-dir")
    .arg(format!("{out_dir}/zig-cache"))
    .arg("--global-cache-dir")
    .arg(format!("{out_dir}/zig-global-cache"))
    .current_dir(manifest_dir);

  let status = cmd
    .status()
    .expect("failed to run zig build — is zig 0.15.2 installed?");

  assert!(status.success(), "zig build failed");
}

fn generate_bindings(manifest_dir: &Path, out_dir: &str) {
  let header = manifest_dir.join("opentui.h");

  let bindings = bindgen::Builder::default()
    .header(header.to_str().unwrap())
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .allowlist_function(".*")
    .allowlist_type(".*")
    .generate()
    .expect("failed to generate bindings");

  let out_path = PathBuf::from(out_dir).join("bindings.rs");
  bindings
    .write_to_file(out_path)
    .expect("failed to write bindings");
}

fn rust_target_to_zig() -> String {
  let target = env::var("TARGET").unwrap();
  let parts: Vec<&str> = target.split('-').collect();
  let arch = parts[0];
  let os = if parts.len() >= 3 { parts[2] } else { parts[1] };

  let zig_arch = match arch {
    "x86_64" => "x86_64",
    "aarch64" => "aarch64",
    other => panic!("unsupported architecture: {other}"),
  };

  let zig_os = match os {
    "darwin" | "macos" => "macos",
    "linux" => "linux",
    "windows" => "windows-gnu",
    other => panic!("unsupported os: {other}"),
  };

  format!("{zig_arch}-{zig_os}")
}
