const path = require('path');

module.exports = {
  entry: "./public/game.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "game.js",
  },
  mode: "development",
  devServer: {
    contentBase: 'public',
    index: 'index.html'
  }
};
