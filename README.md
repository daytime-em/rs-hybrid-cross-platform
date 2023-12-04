# Hybrid Rust/Android/iOS/Web Monorepo (sorta)

A rust project backing three apps: one iOS, one Android, and one Web (wasm + react). The apps find prime numbers and display them a cool chart of the calculated primes.

Each platform package can link to the compiled rust library from its own build, and none of the platform packages depend on each other. This README has some basic setup instructions for each platform. Each of the Gradle, Xcode, and NPM projects have a README with more information, but you shouldn't need it just to get building.

## Setup Instructions

### 1. Install the tools

In order to build for every platform, you'll need to have the following tools installed.

* [rustup](https://www.rust-lang.org/tools/install) for building the rust code.
* [Android Studio](https://developer.android.com/studio) for the android app, with the [NDK](https://developer.android.com/ndk) installed
* [Xcode](https://apps.apple.com/us/app/xcode/id497799835) for the iOS app
* [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm), or yarn I guess, for the web app

### 2. Install rustc targets

You need to install all of the following `rustc` compile targets:

```console
# Building for Android
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android
# Building for iOS
rustup target add aarch64-apple-darwin x86_64-apple-darwin\
 aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
```

To build for WebAssembly, you'll also have to install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

```console
cargo install wasm-pack
```

#### Android-Only

For Android, you'll also need to tell rustc about the NDK with a special `Config.toml`. There's already a template available at [.cargo/config.toml.template](.cargo/config.toml.template). Just save a copy of that file in the same dir as `Config.toml` and then edit the config to point to the given paths on your system.

### 3. Edit/Build the Projects

#### Rust

To edit the Rust project, just **open the root dir** in vscode or your preferred rust editor. There is a workspace in the repo root that has all the rust crates.

#### Android

To build the android project, you only need to open the project at [packages/android/](packages/android/). The rust library will be built for Android when you build the app binary.

#### iOS

To build the android project, you only need to open the project at [packages/ios/ios.xcodeproj/](packages/ios/ios.xcodeproj/). The rust library will be built incrementally every time you build or deploy your iOS project.

#### Web

Building the web project is straightforward. You just need to run the provided `./build-wasm.sh` script

**Setting up the Web Project**:

Just run `./build-wasm.sh install`. It should build the WebAssembly and set up the webapp

**Editing the Web App**:

You can open vscode in the repo root to edit the web app and rust code at once.

The React app is in [packages/web/ts-primes](packages/web/ts-primes/), but vscode will pick up everything properly if you open the repo root dir.

**Running the Web App**:

Just run `./build-wasm.sh start-app` to start a dev server for the web app. Your browser should open automatically.

## Packages

### Rust Library

The core rust library can be found here: [rustlib](packages/rustlib). It's part of a workspace defined at the repo root.

#### Editing and building the Rust library

If you have rust installed, you should be able to build this crate from anywhere in the repo using `cargo build`.

### iOS App

The iOS project can be found here: [iOS side](packages/ios) and [Rust side](packages/rustlib/binding_swift).

### Android App

The Android project can be found here: [Android side](packages/android) and [Rust side](packages/rustlib/binding_ffi)
