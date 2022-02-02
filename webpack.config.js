const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  entry: "./site/index.tsx",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bundle.js",
  },
  module: {
    rules: [
      {
        test: /\.(woff2?)|(ttf)$/,
        type: "asset/resource",
      },
      {
        test: /\.css$/,
        use: ["style-loader", "css-loader"],
      },
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
      {
        test: /\.wasm$/,
        type: "webassembly/async",
      },
    ],
  },
  resolve: {
    extensions: [".ts", ".tsx", ".js", ".wasm"],
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin({
      patterns: [{ from: "./site/index.html", to: "./index.html" }],
    }),
  ],
  devServer: {
    host: "0.0.0.0",
    port: 8080,
  },
  experiments: { asyncWebAssembly: true },
};
