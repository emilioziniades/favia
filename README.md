# favia 🪸

_Still very much a work in progress_

Sometimes I have some content, and I want to stick it into a website. I'd prefer not to wade into some JavaScript framework simply to glue content and html together. I would especially prefer not to write css directly. So I made `favia`.

`favia` is a lightweight static site generator written in Rust. It is zero-config, and has [Tailwind](https://tailwindcss.com/) built-in. Simply describe your [Tera](https://tera.netlify.app/) templates with Tailwind utility classes, as well as your content in markdown, and favia stiches them together.

## api

There are only two commands:

`favia build` builds a bundle of html, css and static files to be served.

`favia dev` runs a development server listening for changes and rebuilding.

You can specify whether you want the output to be verbose, which changes the log level. `favia -v build` includes debug logs, whilst `favia -vv build` includes trace logs. It defaults to info level logs.

## project structure

A favia project has only two directories

- `content`

- `templates`

`content` contains a tree of subdirectories and/or markdown files, optionally with TOML frontmatter, surrounded by `+++`.

`templates` contains a tree of subdirectories and/or [Tera](https://tera.netlify.app/) templates.

favia determines the site structure from these two folders.

## todo

See open issues

## contributing

Please feel free to open an issue if you have found a bug, or, if you want to participate in development, make a ticket describing the issue you're addressing, and reference that ticket in a pull request.
