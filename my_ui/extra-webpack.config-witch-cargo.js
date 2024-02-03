//intendid for use with cargo/rust project directly, not what i'm searching for

const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  entry: "multilang_playground.wasm",
  output: {
    filename: "multilang_playground.wasm",
    path: path.resolve(__dirname, "dist"), // directory of where the bundle will be created at
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: __dirname, // Define where the root of the rust code is located (where the cargo.toml file is located)
    }),
  ],
};
