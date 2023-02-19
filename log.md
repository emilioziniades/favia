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

# 05-02-2022

Error handling is better now. It's question marks all the way to the main function. It required some boilerplate deriving `From` for all the inner error types. But it makes sense doing this in an application project. Maybe some time I will learn more about anyhow and use it better.

I also refactored everything and now that I have done some tidying I am ready to continue implementing `favia build`.

One thing that is bugging me, is how to cross reference frontmatter. Sometimes you'll want to include other files' frontmatter into a different template. Say you have a blog, and you want a home page with a list of blog posts. You would need to somehow grab the frontmatter from all those blog posts. How to do that? One idea I have been toying with is adding a special toml field called "references". In `index.md` you could add `references = "blog"`. Which would then allow you to access the frontmatter from the files in `blog` in the `index.html` template, and iterate over them, or something like that. Sounds like a mission... but doable. I am definitely not there yet, and still need to get the basic build functionality working.

# 06-02-2022

Got a little stuck on error handling. Each depdendency has its own way of representing errors, and it's a trade-off between lots of boilerplate to extract all the details, vs just passing on the higher error without any context. Ideally I'd like to present nice, contextful errors. For development purposes I am just doing `error!("{error:?})` and that is good enough. Trying to not get too stuck up on that detail.

I have generalized the `build` command further. Instead of only building `index.html`, it can now build any number of templates. However, it can't handle nested content structures, which is what's next.

Working on the one-to-many case where one template should create many blog posts. I decided to take an hour and write an integration test, which does an end-to-end test of the build command and checks that the correct files were outputted.

I have immediately realized a problem with this zero-config approach. Consider `content/blog` which has many blog posts. You may want `blog.html`, a page with a list of all your blog posts. But, you also want a page for each individual blog post. There needs to be some convention for how to refer to each. NextJs has a pretty neat way of doing this. You put the blog list page in `templates/blog/index.html` and the individual blog pages at `templates/blog/[frontmatter-item-to-be-the-slug].html`. That works pretty well. And it will force me to consider the case of nested files in `content`.

# 19-02-2022

This have gotten a bit crazy at work, so haven't found much time to work on this. I picked it up again today, and made some good progress. The issue I tackled was handling the situation where one template is responsible for generating multiple output files, based on multiple content files. Favia now handles both situations. There are still a few things I'm not happy with, but progress is being made. It is in need of a refactor - the whole build process is sitting in a single function - ideally I'd like to strip that out and make it easier to unit test. Also, the error handling is bugging me. I'm struggling to bubble up errors in a way that preserves the information regarding what cause the error.

In my mind, I partially created this because I wanted to learn Rust, and also because I wanted to rebuild my personal website, which is currently a GatsbyJS site - yuck. So instead of making this into a fully fledged SSG with all the bells an whistles, I am going to get as far as I need to so that I can rebuild my website. This seems like a reasonable short term goal, and will help me prioritize how I go about this.

The next task I am going to tackle, mainly because it seems like a fun task, is the css. I really want to build tailwind into this thing, without having to resort to JavaScript. I'd like to avoid any JS/Node dependencies. There is this [crate](https://docs.rs/tailwind-css/latest/tailwind_css/index.html) which seems promising. I've just given it a quick read and it seems...like it might work. It's not actively maintained and there are some open issues, but yeah.
