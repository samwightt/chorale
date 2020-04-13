let plugins = ["tailwindcss", "postcss-preset-env"];

const purgecss = [
  "@fullhuman/postcss-purgecss",
  {
    content: ["./**/*.tsx", "./**/*.ts"],
    defaultExtractor: (content) => content.match(/[\w-/:]+(?<!:)/g) || [],
  },
];

module.exports = {
  plugins: [
    "tailwindcss",
    "postcss-preset-env",
    process.env.NODE_ENV === "production" ? purgecss : [],
  ],
};
