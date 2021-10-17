import { Configuration } from "webpack";

const isProduction = (process.env.NODE_ENV === "production");

const configuration: Configuration = {
  mode: isProduction ? "production" : "development",
  entry: "./src/main.ts",
  resolve: {
    extensions: [
      ".ts"
    ]
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        loader: "ts-loader",
        options: {
          compilerOptions: {
            noEmit: false
          }
        }
      }
    ]
  },
  output: {
    library: "index",
    libraryTarget: "umd",
    globalObject: 'this',
  }
}

export default configuration;
