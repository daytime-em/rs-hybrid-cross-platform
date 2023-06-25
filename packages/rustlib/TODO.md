# Project TODO

## Android

* Use cbindgen-android.toml
* Move android-related files into a `platforms/android` folder
* Add a `platforms/android/setup-env.sh`

## iOS

* Complete a setup and document process
* Create a `platforms/ios`
* Add a `platforms/ios/cbindgen-ios.toml` with lang c and the swift header macro thing
* Make sure you can clean the bindings

* swift-bridge book updates
  * `swift-name` in the [functions](https://chinedufn.github.io/swift-bridge/bridge-module/functions/index.html) doc is outdated
    * `#[swift_bridge(swift_name = "printViaPrivateDelegate")]`
  * Swift Script needs updating: Use a Modulemap and make an xcframework

* swift-bridge doesn't support passing closures into rust
* swift-bridge requires your `extern "Swift"` methods to have labels; WAIT this one makes sense
