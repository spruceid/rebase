const webpack = require('webpack');
const path = require('path');

module.exports = {
  entry: './src/index.ts',
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: ['.ts', '.js'],
    alias: {
      process: 'process/browser',
    },

    fallback: {
      crypto: require.resolve('crypto-browserify'),
      http: require.resolve('stream-http'),
      https: require.resolve('https-browserify'),
      os: require.resolve('os-browserify'),
      path: require.resolve('path-browserify'),
      stream: require.resolve('stream-browserify'),
      url: require.resolve('url/'),
    },

  },
  plugins: [
      new webpack.ProvidePlugin({
        process: 'process/browser',
      }),
  ],
  output: {
    filename: 'index.js',
    library: 'index',
    path: path.resolve(__dirname, 'dist'),
  },
  experiments:{ asyncWebAssembly: true }
};