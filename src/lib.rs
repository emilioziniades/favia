use log::{debug, error, info, trace};
use markdown::to_html;
use std::{env, fs, process};
use tera::{Context, Tera};

pub fn dev() {
    info!("development server starting");
    todo!()
}

pub fn build() -> anyhow::Result<()> {
    info!("building site");
    let cwd = env::current_dir()?;

    let templates = cwd.join("templates");
    if !templates.exists() {
        error!("{templates:?} does not exist");
        process::exit(1)
    }

    let content = cwd.join("content");
    if !content.exists() {
        error!("{content:?} does not exist");
        process::exit(1)
    }

    let build_dir = cwd.join(".favia");
    if build_dir.exists() {
        fs::remove_dir_all(build_dir.clone())?;
    }
    fs::create_dir(build_dir.clone())?;

    info!("{cwd:?}");
    info!("{templates:?}");
    info!("{content:?}");

    // TODO: instead of just reading index.html and index.md,
    // recurse through both directories and pair 1-1 or 1-many
    // templates to markdown files

    let tera = match Tera::new(templates.join("**").join("*").to_str().unwrap()) {
        Ok(t) => t,
        Err(e) => {
            error!("failed to parse templates: {e}");
            process::exit(1);
        }
    };

    let index_html = tera.get_template_names().next().unwrap();
    let index_markdown = content.join(index_html).with_extension("md");
    let index_markdown = fs::read_to_string(index_markdown.clone())?;
    let markdown = Markdown::from(index_markdown);
    debug!("markdown: {markdown:#?}");
    let context = Context::from(markdown);
    debug!("{context:#?}");

    let index = fs::File::create(build_dir.join(index_html))?;
    tera.render_to(index_html, &context, index)?;

    Ok(())
}

#[derive(Debug)]
struct Markdown {
    frontmatter: toml::Value,
    content: String,
}

impl From<String> for Markdown {
    fn from(markdown: String) -> Self {
        let frontmatter_start = markdown.find("+++").unwrap();
        let frontmatter_end = markdown.rfind("+++").unwrap();
        let frontmatter = markdown[frontmatter_start + 3..frontmatter_end].to_string();
        let frontmatter = frontmatter.trim();
        let markdown = markdown[frontmatter_end + 3..].to_string();
        let markdown = markdown.trim();

        let frontmatter = frontmatter
            .trim()
            .parse::<toml::Value>()
            .unwrap_or_else(|err| {
                error!("failed to parse frontmatter: {err}");
                process::exit(1)
            });
        let content = to_html(&markdown);
        Self {
            frontmatter,
            content,
        }
    }
}

impl From<Markdown> for tera::Context {
    fn from(markdown: Markdown) -> Self {
        let mut context = Context::new();
        context.insert("content", &markdown.content);
        let value = &tera::to_value(markdown.frontmatter).expect("parse from toml to tera::Value");
        context.insert("fm", value);
        context
    }
}
