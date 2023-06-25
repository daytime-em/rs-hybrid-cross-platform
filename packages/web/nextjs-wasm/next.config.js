/** @type {import('next').NextConfig} */
const nextConfig = {
  webpack: (
    config,
    { buildId, dev, isServer, defaultLoaders, nextRuntime, webpack }
  ) => {
    patchWasmModuleImport(config, isServer);
    return config;
  },
}; /* config options here */

module.exports = nextConfig

function patchWasmModuleImport(config, isServer) {
  config.experiments = Object.assign(config.experiments || {}, {
      asyncWebAssembly: true,
  });

  config.optimization.moduleIds = 'named';

  config.module.rules.push({
      test: /\.wasm$/,
      type: 'webassembly/async',
  });

  // TODO: improve this function -> track https://github.com/vercel/next.js/issues/25852
  if (isServer) {
      config.output.webassemblyModuleFilename = './../static/wasm/41f0d0f0480204d7.wasm';
  } else {
      config.output.webassemblyModuleFilename = 'static/wasm/41f0d0f0480204d7.wasm';
  }
}
