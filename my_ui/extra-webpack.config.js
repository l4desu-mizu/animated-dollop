module.exports = {
    experiments: {
      asyncWebAssembly: true
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
