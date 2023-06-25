# Hybrid Rust/Android/iOS/Web Monorepo (sorta)

A rust project backing three projects: one iOS, one Android, and one Web (wasm + nextjs)

Each platform package has its own toolchain, the simplest one to use. These toolchains can interact with `cargo` to build the rust libraries for the chosen platform.

## Setup

Setup instructions for each platform can be found in the project READMEs. 

### Rust

The core rust library can be found here: [rustlib](packages/rustlib)

### iOS

The iOS project can be found here: [iOS side](packages/ios) and [Rust side](packages/rustlib/binding_swift)

### Android

The Android project can be found here: [Android side](packages/android) and [Rust side](packages/rustlib/binding_ffi)

### iOS

The iOS project can be found here: [web side](packages/web) and [Rust side](packages/rustlib/binding_webpack)
