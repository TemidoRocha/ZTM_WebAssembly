const path = require('path')
const HTMLWepackPlugin = require('html-webpack-plugin')

module.exports = {
  entry: './public/main.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'index.js'
  },
  plugins: [
    new HTMLWepackPlugin({
      template: './public/index.html'
    })
  ]
}