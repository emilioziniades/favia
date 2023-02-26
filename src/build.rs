use crate::directories::Directories;
use crate::error::Error;
use crate::page_data::PageData;
use log::{debug, info, trace};
use std::{fs, io, path::PathBuf};
use tera::Tera;
use walkdir::WalkDir;

pub fn build(cwd: PathBuf) -> Result<(), Error> {
    info!("building site");
    let dirs = Directories::new(cwd)?;

    let tera = Tera::new(dirs.templates.join("**").join("*").to_str().unwrap())?;

    for entry in WalkDir::new(&dirs.content) {
        let entry = entry?;
        if entry.file_type().is_dir() {
            continue;
        }

        let content_path = entry.path();
        debug!("building content file {content_path:#?}");

        // find template path

        let mut template_path = dirs.template_path(content_path);
        let mut build_path = dirs.build_path(template_path.as_ref());

        if template_path.is_some() {
            debug!("template file found: {template_path:#?}");
        } else {
            debug!("no exact template file found");
            let mut factory_template_path = None;
            let template_path_parent = dirs.templates.join(
                content_path
                    .strip_prefix(&dirs.content)
                    .unwrap()
                    .parent()
                    .unwrap(),
            );
            for dir_item in fs::read_dir(template_path_parent)? {
                let path = dir_item?.path();
                if path.is_dir() {
                    continue;
                }
                let file_stem = path.file_stem().unwrap().to_str().unwrap();
                if file_stem.contains('[') && file_stem.contains(']') {
                    if let Some(old_factory_template) = factory_template_path {
                        return Err(Error::Favia(format!(
                            "Multiple factory functions found in single directory: {:#?} and {:#?}",
                            &old_factory_template, &path
                        )));
                    }
                    factory_template_path = Some(path);
                }
            }

            let factory_template_path = factory_template_path.ok_or_else(|| {
                Error::Favia(format!("no factory template found for {content_path:#?}"))
            })?;

            debug!("found factory template: {factory_template_path:#?}");
            template_path = Some(factory_template_path);

            let slug = content_path.file_stem().unwrap();

            build_path = dirs.build_path(
                Some(
                    template_path
                        .as_ref()
                        .unwrap()
                        .parent()
                        .unwrap()
                        .join(slug)
                        .with_extension("html"),
                )
                .as_ref(),
            );
        }

        // render page from content path and template path

        let tera_template_name = dirs.tera_template_name(template_path.as_ref().unwrap());
        if let Some(tera_template_name) = tera
            .get_template_names()
            .find(|name| name == &tera_template_name)
        {
            debug!("tera template found {tera_template_name}");

            let page_data: PageData = fs::read_to_string(content_path)?.try_into()?;
            trace!("parsed markdown: {page_data:#?}");
            let context = tera::Context::from(page_data);
            trace!("tera context: {context:#?}");

            fs::create_dir_all(build_path.as_ref().unwrap().parent().unwrap())?;
            let build_file = fs::File::create(build_path.unwrap())?;

            tera.render_to(tera_template_name, &context, build_file)?;
        } else {
            return Err(
                io::Error::new(io::ErrorKind::NotFound, format!("{tera_template_name:?}")).into(),
            );
        }
    }

    Ok(())
}
