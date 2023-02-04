use crate::directories::Dirs;
use crate::error::Error;
use crate::markdown::Markdown;
use log::{debug, info};
use std::fs;
use tera::Tera;

pub fn build() -> Result<(), Error> {
    info!("building site");
    let dirs = Dirs::new()?;

    let tera = Tera::new(dirs.templates.join("**").join("*").to_str().unwrap())?;

    // TODO: instead of just reading index.html and index.md,
    // recurse through both directories and pair 1-1 or 1-many
    // templates to markdown files

    let index_html = tera.get_template_names().next().unwrap();
    let index_markdown = dirs.content.join(index_html).with_extension("md");
    let index_markdown = fs::read_to_string(index_markdown)?;
    let markdown: Markdown = index_markdown.try_into()?;
    debug!("markdown: {markdown:#?}");
    let context = tera::Context::from(markdown);
    debug!("{context:#?}");

    let index = fs::File::create(dirs.build.join(index_html))?;
    tera.render_to(index_html, &context, index)?;

    Ok(())
}
