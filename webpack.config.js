const CopyWebpackPlugin = require('copy-webpack-plugin')
const path = require('path')

module.exports = {
  entry: './bootstrap.js',
  output: {
    path: path.resolve(__dirname, './docs'),// publishing to GitHub Pages requires /docs or /(root)
    filename: 'bootstrap.js'
  },
  mode: 'development',
  plugins: [
    new CopyWebpackPlugin(['index.html'])
  ]
}
