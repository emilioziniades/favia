# favia 🪸

A lightweight static site generator written in Rust. It is zero-config, and has Tailwind built-in. Simply describe your [Tera](https://tera.netlify.app/) templates, as well as your content in markdown, and favia stiches them together.

_Please note that this project is still a work in progress_

## api

There are only two commands:

`favia build` builds a bundle of html and css to be served

`favia dev` runs a development server listening for changes and rebuilding

You can specify whether you want the output to be verbose, which changes the log level. `favia -v build` includes debug logs, whilst `favia -vv build` includes trace logs. It defaults to info level logs.

## project structure

A favia project has only two directories

- `content`

- `templates`

`content` contains a tree of subdirectories and/or markdown files, optionally with TOML frontmatter, surrounded by `+++`.

`templates` contains a tree of subdirectories and/or [Tera](https://tera.netlify.app/) templates.

favia determines the site structure from these two folders.

## contributing

Please feel free to open an issue if you have found a bug, or, if you want to participate in development, go ahead and make a pull request.
