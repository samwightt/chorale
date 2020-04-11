const purgecss = require('@fullhuman/postcss-purgecss');

let plugins = ['tailwindcss', 'postcss-preset-env']

if (process.env.NODE_ENV === 'production') plugins.push(['@fullhuman/postcss-purgecss', {content: ['./**/*.tsx']}])

module.exports = {
  plugins: plugins
}