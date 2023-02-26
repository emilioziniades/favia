use crate::directories::Directories;
use crate::error::Error;
use crate::page_data::PageData;
use log::{debug, info, trace};
use std::{fs, io, path::PathBuf};
use tera::Tera;
use walkdir::WalkDir;

pub fn build(cwd: PathBuf) -> Result<(), Error> {
    info!("building site");
    info!("preparing directories");
    let dirs = Directories::new(cwd)?;

    let tera = Tera::new(dirs.templates.join("**").join("*").to_str().unwrap())?;

    info!("building files");
    for entry in WalkDir::new(&dirs.content)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
    {
        let content_path = entry.path();
        debug!("content file: {content_path:#?}");

        let template_path = dirs.find_template_path(content_path)?;

        let build_path = dirs.make_build_path(&template_path, content_path);
        debug!("build path: {build_path:?}");

        let tera_template_name = dirs.tera_template_name(&template_path);
        let tera_template_name = tera
            .get_template_names()
            .find(|name| name == &tera_template_name)
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, format!("{tera_template_name:?}"))
            })?;

        debug!("tera template found {tera_template_name}");

        let page_data: PageData = fs::read_to_string(content_path)?.try_into()?;
        trace!("parsed page data: {page_data:#?}");
        let context = tera::Context::from(page_data);
        trace!("tera context: {context:#?}");

        fs::create_dir_all(build_path.parent().unwrap())?;
        let build_file = fs::File::create(build_path)?;

        tera.render_to(tera_template_name, &context, build_file)?;
    }

    Ok(())
}
