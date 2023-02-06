use crate::directories::Directories;
use crate::error::Error;
use crate::markdown::Markdown;
use log::{debug, info, trace};
use std::{fs, io, path::PathBuf};
use tera::Tera;

pub fn build(cwd: PathBuf) -> Result<(), Error> {
    info!("building site");
    let dirs = Directories::new(cwd)?;

    let tera = Tera::new(dirs.templates.join("**").join("*").to_str().unwrap())?;

    // TODO: handle nested case - this assumes flat content directory

    for dir_item in fs::read_dir(&dirs.content)? {
        let content_path = dir_item?.path();

        if content_path.is_dir() {
            // TODO what about here???
            // for now it's fine, until we need to consider nested directories
            continue;
        }
        debug!("building content file: {content_path:?}");
        let template_name = dirs.template_name(&content_path);

        let template_name = tera
            .get_template_names()
            .find(|name| name == &template_name);

        let template_name = match template_name {
            Some(name) => name,
            None => {
                return Err(
                    io::Error::new(io::ErrorKind::NotFound, format!("{template_name:?}")).into(),
                )
            }
        };

        let content_data: Markdown = fs::read_to_string(content_path)?.try_into()?;
        trace!("parsed markdown: {content_data:#?}");

        let context = tera::Context::from(content_data);
        trace!("tera context: {context:#?}");

        let build_name = dirs.build_name(template_name);
        let build_file = fs::File::create(build_name)?;

        tera.render_to(template_name, &context, build_file)?;
    }

    Ok(())
}
