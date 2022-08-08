const path = require('path');
const webpack = require('webpack');

module.exports = {
    entry: './js/index.js',
    output: {
        filename: 'main.js',
        path: path.resolve(__dirname, 'dist'),
    },
    plugins: [
        new webpack.EnvironmentPlugin({ API_URL: 'http://localhost:8000' })
    ],
};