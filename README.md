<p align="center">
  <img width="200" height="200" src="/static/icon.png">
</p>

# Chorale

Chorale is a set of simple, composable, and well-designed tools for interacting with Notion.

- **Simple**: A library shouldn't be the pain point when you're trying to interact with your content. Chorale makes sure that's not the case. Chorale's APIs are small and easy to use while being incredibly fast and efficient.
- **Composable**: All of Chorale's tools are built on top of each other like legos. If you need to customize anything, you can easily swap out one of the pieces for your own.
- **Well-designed**: Chorale is a joy to use. Time is spent designing each of Chorale's APIs, making sure they're not only functional, but that they just feel good to develop with.

Chorale is currently under active development. This page will be updated often, so be sure to check it!

# Things You Can Do With Chorale

- **Interact with the Notion API**: Chorale's Notion API interface lets you easily get content from Notion's private API. It's completely type-safe and is built to run incredibly fast with async I/O.
- **Render Notion content**: Chorale's Notion page renderer can render Notion pages up to 10x faster than the default Notion app; it can render pages in *microseconds*. It's completely modular and customizable: you control the output.
- **Run a rendering server**: Use Notion as a CMS and render pages on demand. Attach a custom domain and custom theme to your Notion page, rendering updates in real-time.
- **Generate static pages**: Use Notion as a CMS to generate static pages with Chorale. Chorale's page generator can efficiently batch page renderings at the same time, generating your site in half the amount of time.

## Supported Blocks

- [x] Text
- [x] Page
- [ ] To-do list
- [x] Heading 1
- [x] Heading 2
- [x] Heading 3
- [x] Bulleted list
- [x] Numbered list
- [x] Toggle list
- [x] Quote
- [x] Divider
- [ ] Link to page
- [ ] Callout
- [ ] Image
- [ ] Web bookmark
- [ ] Video
- [ ] Audio
- [ ] Code
- [ ] File
- [ ] Embed
- [ ] Google Drive
- [ ] Tweet
- [ ] GitHub Gist
- [ ] Google Maps
- [ ] Figma
- [ ] Abstract
- [ ] Invision
- [ ] Framer
- [ ] Whimsical
- [ ] Miro
- [ ] PDF
- [ ] Loom
- [ ] Typeform
- [ ] Codepen
- [ ] Table of Contents
- [ ] Block equation
- [ ] Template button
- [ ] Breadcrumb

## Supported Formatting
- [x] Bold
- [x] Italicize
- [x] Underline
- [x] Strikethrough
- [x] Code
- [x] Link
- [x] Color
- [ ] Inline Math
- [ ] Mentions
- [ ] Comments

# Running Locally

Chorale hasn't been published to Crates.io yet, so to work with it, you'll need to download the workspace. Here's the commands to do that (make sure you have the latest version of Rust and Cargo installed!):

```sh
    git clone https://github.com/samwightt/chorale-renderer -b develop
    cd chorale-renderer
    cargo build
```

To run the testing crate:

```sh
    cd testing
    cargo run
```

To run the benchmarks:
```sh
    cd testing
    cargo bench
```

# License Info

Chorale: A blazing fast Notion page renderer.
Copyright (C) 2020 Sam Wight

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
