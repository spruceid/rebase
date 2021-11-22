const webpack = require('webpack');
const path = require('path');

module.exports = {
  entry: './src/issuer.ts',
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
    // alias: {
    //   process: 'process/browser',
    // },

    fallback: {
      browser: false,
      // buffer: require.resolve('buffer/'),
      buffer: false,
      // crypto: require.resolve('crypto-browserify'),
      crypto: false,
      // http: require.resolve('stream-http'),
      http: false,
      //   https: require.resolve('https-browserify'),
      fs: false,
      net: false,
      // os: require.resolve('os-browserify'),
      os: false,
      // path: require.resolve('path-browserify'),
      path: false,
      // stream: require.resolve('stream-browserify'),
      stream: false,
      // util: require.resolve('util/'),
      util: false,
      // url: require.resolve('url/'),
      url: false,
      zlib: false
    },
  },
  // plugins: [
  //     new webpack.ProvidePlugin({
  //       process: 'process/browser',
  //     }),
  // ],
  output: {
    filename: 'issuer.js',
    library: 'issuer',
    path: path.resolve(__dirname, 'dist'),
  },
  experiments:{ asyncWebAssembly: true }
};