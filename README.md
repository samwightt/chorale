<p align="center">
  <img width="200" height="200" src="/static/icon.png">
</p>

# Chorale

Chorale is a blazing-fast Notion renderer that makes your content sing. It's built to be as simple and as flexible as possible. 

Chorale renders Notion content statically to a whatever type you want. The default `ui` crate renders it to a string, which allows Chorale to render the contents of a large Notion page in *microseconds*.

Chorale 2 is currently under active development. This page will be updated often, so be sure to check it!

## Supported Blocks

- [x] Text
- [x] Page
- [ ] To-do list
- [ ] Heading 1
- [ ] Heading 2
- [ ] Heading 3
- [x] Bulleted list
- [x] Numbered list
- [x] Toggle list
- [ ] Quote
- [ ] Divider
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
- [ ] Link
- [ ] Inline Math
- [ ] Color
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
