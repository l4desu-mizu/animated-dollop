module.exports = {
  experiments: {
    futureDefaults: true, // enables most wasm features as they should be in future (e.g. asyncWebAssembly)
    outputModule: true,
    topLevelAwait: true
  },
  module: {
    rules: [
        {
            test: /\.wasm$/,
            type: "asset/inline",
        },
    ],
  }
};
