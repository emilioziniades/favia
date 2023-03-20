use crate::page_data::PageData;
use crate::Result;
use crate::{builder::Builder, directories::Directories};
use log::{debug, info, trace};
use std::{fs, path::Path};
use walkdir::WalkDir;

pub fn build(cwd: &Path) -> Result<()> {
    info!("building site");
    info!("preparing directories");
    let builder = Builder::new(cwd)?;
    let dirs = Directories::new(cwd)?;

    info!("building files");
    for entry in WalkDir::new(&dirs.content)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
    {
        let content_path = entry.path();
        debug!("content file: {content_path:#?}");

        build_content_file(content_path, &builder)?;
    }

    Ok(())
}

pub fn build_content_file(content_path: &Path, builder: &Builder) -> Result<()> {
    debug!("content file: {content_path:#?}");

    let template_path = builder.template_path(content_path)?;

    let build_path = builder.build_path(&template_path, content_path);
    debug!("build path: {build_path:?}");

    let tera_template_name = builder.tera_template_name(&template_path)?;
    debug!("tera template found {tera_template_name:?}");

    let page_data: PageData = fs::read_to_string(content_path)?.try_into()?;
    trace!("parsed page data: {page_data:#?}");

    let context = tera::Context::from(page_data);
    trace!("tera context: {context:#?}");

    builder.render(&tera_template_name, context, &build_path)?;

    Ok(())
}
