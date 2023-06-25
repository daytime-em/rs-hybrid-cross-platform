#[allow(clippy::redundant_static_lifetimes)]
const XCODE_CONFIGURATION_ENV: &'static str = "CONFIGURATION";

fn main() {
  // TODO: This could be configurable somehow, like maybe the Cargo.toml if possible

  // setldflags 
  println!("cargo:rustc-link-arg=-lSystem");
  //println!("cargo:rustc-link-arg=-liconv");

  // Swift Bridge
  let out_dir = "../../ios/Generated";

  let bridges = vec!["src/lib.rs"];
  for path in &bridges {
      println!("cargo:rerun-if-changed={}", path);
  }
  println!("cargo:rerun-if-env-changed={}", XCODE_CONFIGURATION_ENV);

  swift_bridge_build::parse_bridges(bridges)
      .write_all_concatenated(out_dir, env!("CARGO_PKG_NAME"));
}
