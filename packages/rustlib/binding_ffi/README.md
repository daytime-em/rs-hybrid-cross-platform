# Rust Android/FFI Example

## Setup

### Install the android Rust compilation targets

```console
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android
```

### Use the NDK's linker to link properly to the android ndk env

Put this in `.cargo/config.toml`

```toml
[target.aarch64-linux-android]
linker = "/Users/edixon/Library/Android/sdk/ndk/25.2.9519653/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android21-clang"

[target.armv7-linux-androideabi]
linker = "/Users/edixon/Library/Android/sdk/ndk/25.2.9519653/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi21-clang"
```


### Build with cargo

Just do `cargo build`
