module.exports = {
  experiments: {
    futureDefaults: true, // enables most wasm features as they should be in future (e.g. asyncWebAssembly)
    outputModule: true,
    topLevelAwait: true
  },
  module: {
    rules: [ // this was mentioned to include wasm files and inline them... haven't seen it work
        {
            test: /\.wasm$/,
            type: "asset/inline",
        },
    ],
  },
  resolve: {  // tries to help with including wasm files, although this seems to be an older trick where wasm-bindgen/wasm-pack did not add .wasm to the imported file, thus violating webpack logic
    extensions:['.wasm']
  }
};
