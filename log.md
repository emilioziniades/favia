This log serves as a journal of development progress, in an attempt to share openly and to document.

# 03-02-2022

First day building this. A short summary of what this project aims to acheive:

- [ ] zero config - determine pages from structure of `content` and `templates` folder
- [ ] tailwind built in, no node dependency
- [x] simple api: `favia dev` and `favia build`
- [ ] live rebuild on file changes
- [ ] explicit logging, warning of unused or missing template variables
- [ ] toml frontmatter parsing

So far, I have set up the skeleton of the cli with `clap`, which is basically an empty shell. It has some basic documentation, and is published on crates.io

The next step is to build out the `build` command. It should essentially take a directory of contents, and a directory of templates, and mesh those two together to form a bundle of html files. The simplest is a single template. The next step would be a base template and a content template. After that, try and load toml frontmatter from index.md and put it into the corresponding index.html template.
