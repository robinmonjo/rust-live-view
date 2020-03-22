const path = require('path');

module.exports = {

  entry: './assets/js/index.js',

  output: {
    path: path.resolve(__dirname, 'assets/dist'),
    filename: 'bundle.js'
  },

  module: {
    rules: [
      {
        test: /\.js$/,
        exclude: /(node_modules)/,
        use: {
          loader: 'babel-loader',
          options: {
            presets: ['@babel/preset-env']
          }
        }
      }
    ]
  },

  mode: 'development'
};
