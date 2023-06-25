# Rust Library

## Cross-Compilation Setup

You need to install some targets in order to cross-compile the libs.

You probably only need to do this when building for release or if delegated-to from the build

### Android Setup

You need to install an NDK and then configure the cross-compile targets before you can build for Android.

based on a mozilla blog (TODO: link)

#### Configure Targets

You need to install the Android targets you want to build for. This project assumes you want both 32 and 64-bit arm binaries, and that you only need v7 and up

You aren't supposed to check in the `target`s, instead I guess the dev/CI should take care of it

```console
rustup target add armv7-linux-androideabi
rustup target add aarch64-linux-android 

```

Targets also have a config.toml associated with them, since they have their own linker/ar.
These are in the NDK install dir, which the dev/CI must provide.

Here's a toml template for the Android targets. This differs from the mozilla blog, which is out of date

Put this in `.cargo/config.toml`

```toml
[target.aarch64-linux-android]
linker = "/Users/edixon/Library/Android/sdk/ndk/25.2.9519653/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android21-clang"

[target.armv7-linux-androideabi]
linker = "/Users/edixon/Library/Android/sdk/ndk/25.2.9519653/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi21-clang"
```

#### Build Targets

Build with cargo. For an open-source lib it might make sense to build x86 for people running tests on intel hw

```console
cargo build --target aarch64-linux-android 
cargo build --target armv7-linux-androideabi

```

#### Dynamic or static

The mozilla blog says to make `dylib`s because it's going to use `System.loadLibrary` () on the rust artifact, but we have a different setup.

In fact, a dynamic lib would be too big. And `cbindgen` doesn't support it. Instead we make a statically-linked lib that the downstream project can use. The downstream project itself may be a dynamic lib (for android always a `.so`).
