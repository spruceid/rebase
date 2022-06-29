const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const path = require('path');
const sveltePreprocess = require('svelte-preprocess');
const webpack = require('webpack');
require('dotenv').config({ path: './.env' });

const mode = process.env.NODE_ENV || 'development';
const prod = mode === 'production';

module.exports = {
	entry: {
		'build/bundle': ['./src/main.ts'],
	},
	resolve: {
		alias: {
			svelte: path.dirname(require.resolve('svelte/package.json')),
			components: path.resolve(__dirname, 'src/components'),
			routes: path.resolve(__dirname, 'src/routes'),
			utils: path.resolve(__dirname, 'src/util'),
			src: path.resolve(__dirname, 'src/'),
		},
		extensions: ['.mjs', '.js', '.ts', '.svelte'],
		mainFields: ['svelte', 'browser', 'module', 'main'],
		fallback: {
			os: require.resolve('os-browserify'),
			url: require.resolve('url/'),
			https: require.resolve('https-browserify'),
			assert: require.resolve('assert/'),
			http: require.resolve('stream-http'),
			stream: require.resolve('stream-browserify'),
			buffer: require.resolve('buffer/'),
			crypto: require.resolve('crypto-browserify'),
			path: false,
			fs: false,
		},
	},
	output: {
		path: path.join(__dirname, '/public'),
		filename: '[name].js',
		chunkFilename: '[name].[id].js',
	},
	module: {
		rules: [
			{
				test: /\.ts$/,
				loader: 'ts-loader',
				exclude: /node_modules/,
			},
			{
				test: /\.svelte$/,
				use: {
					loader: 'svelte-loader',
					options: {
						compilerOptions: {
							dev: !prod,
						},
						emitCss: prod,
						hotReload: !prod,
						preprocess: sveltePreprocess({
							sourceMap: !prod,
							postcss: true,
						}),
					},
				},
			},
			{
				test: /\.css$/,
				use: [MiniCssExtractPlugin.loader, 'css-loader', 'postcss-loader'],
			},
			{
				test: /\.s[ac]ss$/i,
				use: ['style-loader', 'css-loader', 'sass-loader', 'postcss-loader'],
			},
			{
				// required to prevent errors from Svelte on Webpack 5+
				test: /node_modules\/svelte\/.*\.mjs$/,
				resolve: {
					fullySpecified: false,
				},
			},
			{
				test: /\.wasm$/,
				type: 'webassembly/async',
			},
		],
	},
	mode,
	plugins: [
		new webpack.ProvidePlugin({
			Buffer: ['buffer', 'Buffer'],
		}),
		new MiniCssExtractPlugin({
			filename: '[name].css',
		}),
		new webpack.DefinePlugin({
			BUILD_MODE_DEV: !prod,
		}),
		new webpack.DefinePlugin({
			"process.env": JSON.stringify(process.env)
		}),
	],
	devtool: prod ? false : 'source-map',
	devServer: {
		hot: true,
		historyApiFallback: {
			index: '/index.html',
		},
	},
	experiments: {
		asyncWebAssembly: true,
	},
};
