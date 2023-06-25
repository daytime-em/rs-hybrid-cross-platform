# Implementation Notes

## Android

Most of the notes for android are already in the README, but eventually we'll iterate on it

## iOS

### Cargo setup

We need to add some targets. This project probably only needs arm64 and x86_64. If 32bit arm is supported, give it a try tho

```console
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
rustup target add x86_64-apple-ios

```

### swift-bridge

Try this out and add notes to this section

[xcode setup](https://chinedufn.github.io/swift-bridge/building/xcode-and-cargo/index.html)

* Many types are transparently bridged and you don't have to eff with the ffi at all.
* The binding files only change if you actually change the bindings in the lib
* The instructions could be different potentially, to cut out `cargo lipo`
* Modern examples do this

### wasm

`binding_wasm`
Try Parcel as doc'd and see if it it's acceptable
NPM Workspaces: Figure out

Parcel not acceptable. Parcel 2 doesn't support rust

We can probably just run with `wasm-pack`
Yeah fuck that, none of that shit works.

`wasm-pack build --target bundler` also `-d` for output dir
`wasm-pack build --target web` to deploy directly, but I think we want upstream to bundle/webpack/whatever it

`npm link` -> 'publish' locally then `npm link binding_wasm` in another project to link them.

Instead of that, can put the pkg in an npm workspace, ideally the npm projects that depend on `binding_wasm` can invoke `wasm-pack build` as part of pkg resolution

Wasm deployment advice: https://rustwasm.github.io/wasm-bindgen/reference/deployment.html

But also try to make the downstream build system do the `wasm-pack`:
https://docs.npmjs.com/cli/v9/using-npm/scripts  You can do this with `scripts` in package.json! They're in shell, so run that wasm-pack bb

So, we'd want a package.json with a script that builds the "pkg" part.
```json
{
  "scripts": {
    // NOTE: -d is relative to the manifest path
    "prepare": "wasm-pack build --target bundler -d ./path/from/manifest/back/here/binding)wasm ./path/to/rustlib/binding_wasm/Cargo.toml",
  },
  "dependencies": {
    "binding_wasm": "file:./binding_wasm"
  }
}
```

```
packages/
    web/
        rustlib-ts/
            package.json <- Depends on the wasm package by src, "scripts/prepare" will build that package
                wasm-pkg/ <- referenced in above package.json and built by its scripts
```

### Wasm The Right way?

I want a hybrid application:

https://rustwasm.github.io/wasm-pack/book/tutorials/hybrid-applications-with-webpack/

### Web Libs

Gotta do module.exports.js and some other stuff

* TODO - Do I need sideEffects in `binding-webpack` so things that depend locally will recompile nicely?
* So, Next is using webpack 5.. I think that means I too must use webpack 5. It should support wasm tho

Webpack 5 builds fine but I still don't understand export and import..

* Attempted import error: 'rustlib' does not contain a default export (imported as 'greetFromRust').

Ok now I figured it out sorta. You need `import { name }` without a default export, so there's a place to put what you imported I guess



### Random notes

* MONOREPO STUFF:
  * Since repos kind of interdepend, monorepo files could be all defined at Repo-Root
  * This way, everything can be included without having to "../"
  * Does vscode handle this ok?

* cbindgen can't find `swift_macro_name` until (presumably) targets are installed (?)
* oo look theres a project that does this: <https://github.com/chinedufn/swift-bridge>
* Consider whether we want the FFI bindings to be created by the swift/kt project or the rs project
  * I'm leaning toward having the caller project do it, and the FFI be general.
  * swift-bridge requires that we create a special ffi module with swift-oriented annotations and stuff tho
  * Really no benefit to that, we just need the c++/jni tooling to run cbindgen & we can use corrosion
* Possible organization (don't do this all at once):
  * `platforms/cxx`: Has:
    * config for generating c++ bindings for the ffi module
    * `setup-env.sh`
    * ffi crate that depends on pure `rustlib`, written with c++ in mind
    * Android interacts with cxx with JNI bindings that it maintains
    * Remember to mostly lean on exporting symbols transparently from the base package with `extern "C" { type X; fn xyz() -> A; }`

  * `platforms/swift`: Has a module (entire crate?) with swift ffi bindings
    * `setup-env.sh`
    * ffi crate that depends on pure `rustlib`, that uses `swift-bridge`
    * iOS can interact with this using the instructions [here](https://chinedufn.github.io/swift-bridge/building/xcode-and-cargo/index.html)
    * Remember to mostly lean on exporting symbols transparently from the base package with `extern "Rust" { type X; fn xyz() -> A; }`
      * types can be imported from swift also, but it feels better to call generically via ffi, I dunno
* We may want an `ffi-base` crate that provides types for the `platforms` crates. That sounds pretty reasonable