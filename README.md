# Mobile Rust Monorepo (sorta)

Just a quick monorepo that has a rust project backing two mobile projects, one iOS and one Android

## Monorepo tools

Each platform package has its own toolchain, the simplest one to use. These toolchains can interact with `cargo` to build the rust libraries for the chosen platform.

You could probably make a rust workspace with a custom `build.rs` script that can invoke the webpack/xcode/gradle. Cargo is probably capable of this, but I think that strategy would be of limited usefulness when you could just make a GitHub Actions workflow for each target platform.

