import { Configuration } from "webpack";
import path from "path";

console.log("Using shared configuration");
console.log((__filename));

const configuration: Configuration = {
  mode: "development",
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
    libraryExport: "handler",
    libraryTarget: "umd",
    globalObject: 'this',
  }
}

export default configuration;
