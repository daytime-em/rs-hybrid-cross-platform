const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

// TODO - Update to Webpack 5 and export as ESM
module.exports = {
  mode: "production",
  entry: {
    index: "./js/binding_webpack.js",
  },
  experiments: {
    asyncWebAssembly: true,
  },
  output: {
    path: dist,
    filename: "[name].js",
    globalObject: "this",
    library: {
      name: "rustlib",
      type: "umd",
    },
  },
  externals: {
    lodash: {
      commonjs: "lodash",
      commonjs2: "lodash",
      amd: "lodash",
      root: "_",
    },
  },
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new CopyPlugin({
      patterns: [path.resolve(__dirname, "static")],
    }),

    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ],
};
