const path = require('path')
const HTMLWepackPlugin = require('html-webpack-plugin')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

module.exports = {
  entry: './public/main.js',
  output: {
    path: path.resolve(__dirname, 'dist'), // the path for this property needs to be full system path
    filename: 'index.js'
  },
  plugins: [
    new HTMLWepackPlugin({
      template: './public/index.html'
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, './') // the path for this property needs to be full system path
    })
  ],
  experiments: {
    asyncWebAssembly: true
  }
}