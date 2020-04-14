# Chorale

Chorale is a Notion renderer that makes it easy to share public Notion articles that look good and load _extremely_ fast.

Chorale is built using Next.JS and hosted on Zeit Now. A few details:

1. **Chorale is faster than vanilla Notion.** Chorale renders pages 2 to 3 times faster than a regular Public Notion page.
2. **Chorale allows custom domain support.** If you host Chorale yourself, you can easily hook up your own custom domain.
3. **Chorale allows custom theming.** Chorale is open source and allows easy changing of block formatting.
4. **Chorale updates in real time without requiring a rebuild.**
5. **Caches content efficiently and serves all content from the edge.**

[GitHub Link](https://github.com/samwightt/chorale-renderer)

# How to use

Chorale only works with published Notion pages, not private pages. To make your page public, go to the Share menu and click on 'Public Access'. Then click 'Copy Link'.

Your link will look something like this:

[https://www.notion.so/Chorale-ef28925f63894c1d962da11c86879897](https://www.notion.so/Chorale-ef28925f63894c1d962da11c86879897)

Simply replace `[www.notion.so](http://www.notion.so)` with `[chorale.app](http://chorale.app)` and view the page in your web browser. Here's the example link for this page:

[https:/chorale.app/Chorale-ef28925f63894c1d962da11c86879897](http://chorale.app/Chorale-ef28925f63894c1d962da11c86879897)

# How it works

Chorale is built using Next.JS and hosted on Zeit Now. Thanks to Now's **[Serverless Prerendering](https://zeit.co/blog/serverless-pre-rendering)** feature, Chorale can render Notion pages on each visit while still caching the results on the edge. This means that it gets the speed benefits of a build system, but still can render content dynamically.

Chorale is built to be as fast as possible. It scores extremely high on speed tests.

Chorale is still in the early stages of development and has a long time to go.

# Supported Blocks

- Text
- Heading 1
- Heading 2
- Heading 3
- Bulleted List
- Numbered List
- Columns
- Quote Block
- Divider
- Image (early)

# Roadmap

- Implementing image support with exact sizing.
- Adding page covers and page icons.
- Improve design of titles and pages.
- Implementing page links.
- Implementing all other non-embeded and non-database blocks.
- Implementing database blocks/views.
- Implementing some kind of blogging support.

## Changelog

- 2/11/2020:
  - Added support for images
  - Added better code segment rendering
  - Added better column support
  - Better Typescript types for API responses.
  - Better rendering speed.
  - Smaller bundle size with better `font-family` that doesn't require Google Fonts.
  - Added block and text color support (partially, doesn't work with backgrounds).
  - Added divider support
  - Added early image support
- 2/10/2020:
  - Initial version.

# License Info

Chorale: A blazing fast Notion page renderer.
Copyright (C) 2020 Sam Wight

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
