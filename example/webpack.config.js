const path = require('path');
const MonacoWebpackPlugin = require('monaco-editor-webpack-plugin');

module.exports = {
  entry: './index.ts',
  output: {
    path: path.resolve(__dirname, '.'),
    filename: 'index.js',
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: ['ts-loader'],
      },
      {
        test: /\.wasm$/,
        type: 'webassembly/async',
      },
      {
        test: /\.css$/i,
        include: path.resolve(__dirname, '.'),
        use: ['style-loader', 'css-loader', 'sass-loader', 'postcss-loader'],
      },
    ],
  },
  experiments: {
    asyncWebAssembly: true,
  },
  mode: 'development',
  devServer: {
    contentBase: './',
  },
  plugins: [new MonacoWebpackPlugin()],
};
