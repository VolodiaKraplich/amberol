use glib_build_tools::compile_resources;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=Cargo.toml");
  println!("cargo:rerun-if-changed=.git/HEAD");
  println!("cargo:rerun-if-changed=src/amberol.gresource.xml");
  println!("cargo:rerun-if-changed=po/");

  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
  let src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

  // Determine if this is a development build
  let is_development = env::var("CARGO_CFG_DEBUG_ASSERTIONS").is_ok();

  let (version_suffix, application_id) = if is_development {
    (get_version_suffix(), "io.bassi.Amberol.Devel")
  } else {
    (String::new(), "io.bassi.Amberol")
  };

  let version = env::var("CARGO_PKG_VERSION").unwrap();
  let full_version = format!("{}{}", version, version_suffix);
  let gettext_package = "amberol";

  // Pass the configuration to rustc via environment variables
  println!("cargo:rustc-env=APP_VERSION={}", full_version);
  println!("cargo:rustc-env=APPLICATION_ID={}", application_id);
  println!("cargo:rustc-env=GETTEXT_PACKAGE={}", gettext_package);
  println!(
    "cargo:rustc-env=PROFILE={}",
    if is_development {
      "development"
    } else {
      "default"
    }
  );

  compile_translations(&out_dir, &src_dir, gettext_package);
  compile_gresources(&out_dir, &src_dir);
}

/// Gets a version suffix from the git hash for development builds.
fn get_version_suffix() -> String {
  if Path::new(".git").exists()
    && let Ok(output) = Command::new("git")
      .args(["rev-parse", "--short", "HEAD"])
      .output()
    && output.status.success()
  {
    let hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if !hash.is_empty() {
      return format!("-g{}", hash);
    }
  }
  "-devel".to_string()
}

/// Finds .po files in the `po/` directory and compiles them into .mo files.
fn compile_translations(out_dir: &Path, src_dir: &Path, package: &str) {
  let po_dir = src_dir.join("po");
  if !po_dir.exists() {
    println!("cargo:warning='po' directory not found, skipping translation compilation.");
    return;
  }

  let locale_dest = out_dir.join("locale");

  if let Ok(entries) = fs::read_dir(po_dir) {
    for entry in entries.flatten() {
      let path = entry.path();
      if path.is_file() && path.extension().map_or_else(|| false, |e| e == "po") {
        let lang = path.file_stem().unwrap().to_str().unwrap();
        let target_dir = locale_dest.join(lang).join("LC_MESSAGES");
        fs::create_dir_all(&target_dir).expect("Failed to create directory for .mo file");
        let mo_file = target_dir.join(format!("{}.mo", package));

        let status = Command::new("msgfmt")
          .arg("--output-file")
          .arg(&mo_file)
          .arg(&path)
          .status();

        match status {
          Ok(st) if st.success() => {}
          _ => eprintln!(
            "Warning: Failed to run msgfmt for language '{}'. Make sure the gettext package is installed.",
            lang
          ),
        }
      }
    }
  }
}

/// Compiles the GResource bundle.
fn compile_gresources(out_dir: &Path, src_dir: &Path) {
  compile_resources(
    &[
      src_dir.join("src"),
      src_dir.join("src/gtk"),
      out_dir.to_path_buf(),
    ],
    "src/amberol.gresource.xml",
    "amberol.gresource",
  );
}
