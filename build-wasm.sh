#! /bin/bash
# build-wasm.sh: Builds and launches the WebAssembly project
# You could probably use like turborepo or something to do this, 
# but our example is pretty small

function build-wasm-module {
  echo "🕸️🦀 Building wasm + bindings"
  pushd "./packages/web/rustlib-webpack-ts"
  wasm-pack build && popd || popd
}

function build-app {
  echo "🕸️📦🦀 Building wasm + bindings"
  npm run build --workspace="ts-primes"
}

function start-app {
  echo "🕸️🦀🖥️  Starting dev server"
  npm run start --workspace="ts-primes"
}

function install {
  rm -rf ./node_modules
  rm package-lock.json
  build-wasm-module
  npm install
}

readonly SCRIPT=$1

if [ -z $SCRIPT ]; then
  echo "no argument specified. building wasm & app"
  build-wasm-module
  build-app

else
  case $SCRIPT in 
    install)
      install
      ;;
    build-wasm) 
      build-wasm-module
      ;;
    build-app)
      build-app
      ;;
    start-app)
      build-wasm-module
      start-app
      ;;
  esac
fi
