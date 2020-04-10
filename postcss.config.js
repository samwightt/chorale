const purgecss = require('@fullhuman/postcss-purgecss');

module.exports = {
  plugins: ['tailwindcss', 'postcss-preset-env', /*['@fullhuman/postcss-purgecss', {content: ['./**//*.tsx']}]*/]
}