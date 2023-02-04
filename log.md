This log serves as a journal of development progress, in an attempt to share openly and to document.

# 03-02-2022

First day building this. A short summary of what this project aims to acheive:

- zero config - determine pages from structure of `content` and `templates` folder
- tailwind built in, no node dependency
- simple api: `favia dev` and `favia build`
- live rebuild on file changes
- explicit logging, warning of unused or missing template variables
- toml frontmatter parsing

So far, I have set up the skeleton of the cli with `clap`, which is basically an empty shell. It has some basic documentation, and is published on crates.io

The next step is to build out the `build` command. It should essentially take a directory of contents, and a directory of templates, and mesh those two together to form a bundle of html files. The simplest is a single template. The next step would be a base template and a content template. After that, try and load toml frontmatter from index.md and put it into the corresponding index.html template.

# 04-02-2022

Working on the zero-config nature of this. The code now parses toml frontmatter into tera context, and renders the tera template with that context. This means you can simply specify a template and its corresponding markdown content, and favia stiches them together for a single `index.html` template.

The next step is to generalize this for _any_ content and template folders. There are two cases to consider. The first is where there is a one-to-one mapping between a template and content. For example: `about.md` and `about.html`. But, there is also the one-to-many case, where a single template (`blog.html`) maps to several content files in a `blog` directory. In this case, favia should render several pages of the form `blog/post-one.html`, `blog/post-two.html` and so on. The one-to-one case is the easier one so I'll handle it first.

The error handling also leaves something to be desired. I am currently just using `anyhow`, but because there are a limited number of error types (`io`, `toml`, `tera`), it might make sense to create my own error type and derive `From` for all the errors, because in each case the message to be extracted should be different. But, that's for another time.

Sometimes, there will be a template that doesn't have content directly. Something like a `base.html` which is extended by other templates. In that case, you just don't render any pages for that template, and let tera handle the extending.

Also, I should write some tests.
