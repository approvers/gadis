import path from "path";
import sass from "sass";
import fibers from "fibers";
import { Configuration as WebpackConfiguration } from "webpack";
import { Configuration as WebpackDevServerConfiguration } from "webpack-dev-server";
import CopyWebpackPlugin from "copy-webpack-plugin";
import HtmlWebpackPlugin from "html-webpack-plugin";

interface Configuration extends WebpackConfiguration {
  devServer?: WebpackDevServerConfiguration
}

const isProduction = (process.env.NODE_ENV === "production");

const configuration: Configuration = {
  mode: isProduction ? "production" : "development",
  entry: "./src/index.tsx",
  resolve: {
    alias: {
      "~": path.resolve(__dirname, "src")
    }
  },
  module: {
    rules: [
      {
        test: /\.tsx$/,
        use: [
          "babel-loader"
        ]
      },
      {
        test: /\.s?css$/,
        use: [
          "style-loader",
          {
            loader: "css-loader?modules",
            options: {
              sourceMap: !isProduction,
              importLoaders: 2,
              modules: {
                exportLocalsConvention: "dashes"
              }
            }
          },
          {
            loader: "sass-loader",
            options: {
              implementation: sass,
              sourceMap: !isProduction,
            }
          }
        ]
      },
    ]
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, "src", "template.html"),
      scriptLoading: "defer"
    }),
    new CopyWebpackPlugin({
      patterns: [
        {
          from: path.resolve(__dirname, "static"),
          to: path.resolve(__dirname, "dist/static"),
          noErrorOnMissing: true
        }
      ]
    })
  ],
  output: {
    filename: "[contenthash].js",
  },
  devServer: {
    static: path.join(__dirname, "dist"),
    compress: true,
    port: 4000,
    historyApiFallback: true
  }
}

export default configuration;
