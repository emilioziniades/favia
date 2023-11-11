# `favia` 🪸

**NOTE: I am no longer developing this project. If you are looking for a static site generator written in Rust that has all the features described below and much, much, much more, I recommend you check out [zola](https://getzola.org). **

Sometimes I have some content, and I want to stick it into a website. I'd prefer not to wade into some JavaScript framework simply to glue content and html together. So I made `favia`.

`favia` is a lightweight static site generator written in Rust. It is zero-config. Simply describe your [Tera](https://tera.netlify.app/) templates, as well as your content in markdown, and `favia` stiches them together.

## quickstart

First, ensure you have the Rust toolchain installed, which can be done using `rustup`. See [here](https://www.rust-lang.org/tools/install) for more details.

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then, install `favia` using `cargo`

```
$ cargo install favia
```

Finally, create a new project with the `favia new` command. The below command creates a project called `my_favia_project`

```
$ favia new my_favia_project
```

Then, you can run a development server, and start hacking.

```
$ cd my_favia_project && favia develop
```

## api

There are three commands:

`favia build` builds a bundle of html, css and static files to be served. The build output is stored in the `.favia` directory.

`favia develop` runs a development server listening for changes and rebuilding.

`favia new PROJECT_NAME` creates a new project with the specified name and all the required folders and files to get started.

You can specify whether you want the output to be verbose, which changes the log level. `favia -v build` includes debug logs, whilst `favia -vv build` includes trace logs. It defaults to info level logs.

## project structure

A `favia` project has three directories

```
├── content
├── templates
└── static
```

`content` contains a tree of subdirectories and/or markdown files, optionally with TOML frontmatter, surrounded by `+++`.

`templates` contains a tree of subdirectories and/or [Tera](https://tera.netlify.app/) templates.

`static` contains all static files, including css files, images and fonts.

`favia` determines the site structure from these two folders.

## todo

See open issues

## contributing

Please feel free to open an issue if you have found a bug, or, if you want to participate in development, make a ticket describing the issue you're addressing, and reference that ticket in a pull request.
