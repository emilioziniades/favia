use crate::directories::Directories;
use crate::error::Error;
use crate::markdown::Markdown;
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
        let mut build_path = dirs.build_path(&template_path);

        let content_data: Markdown = fs::read_to_string(content_path)?.try_into()?;
        trace!("parsed markdown: {content_data:#?}");

        if template_path.exists() {
            debug!("template file found: {template_path:#?}");
        } else {
            debug!("no exact template file found");
            let mut factory_template_path = None;
            for dir_item in fs::read_dir(&template_path.parent().unwrap())? {
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

            let factory_template_path = factory_template_path.ok_or(Error::Favia(format!(
                "no factory template found for {content_path:#?}"
            )))?;

            debug!("found factory template: {factory_template_path:#?}");
            template_path = factory_template_path;

            let name_to_replace = template_path.file_name().unwrap().to_str().unwrap();
            let left_bracket_pos = name_to_replace.find('[').unwrap();
            let right_bracket_pos = name_to_replace.find(']').unwrap();
            let field_for_slug =
                name_to_replace[left_bracket_pos + 1..right_bracket_pos].to_string();

            let slug = match content_data.get_value(&field_for_slug)? {
                toml::Value::String(value) => value,
                _ => {
                    return Err(Error::Favia(format!(
                        "slug field {field_for_slug} exists, but key is not a string"
                    )))
                }
            };

            build_path = build_path.with_file_name(slug).with_extension("html");
        }

        // render page from content path and template path

        let tera_template_name = dirs.tera_template_name(&template_path);
        if let Some(tera_template_name) = tera
            .get_template_names()
            .find(|name| name == &tera_template_name)
        {
            debug!("tera template found {tera_template_name}");

            let context = tera::Context::from(content_data);
            trace!("tera context: {context:#?}");

            fs::create_dir_all(&build_path.parent().unwrap())?;
            let build_file = fs::File::create(build_path)?;

            tera.render_to(tera_template_name, &context, build_file)?;
        } else {
            return Err(
                io::Error::new(io::ErrorKind::NotFound, format!("{tera_template_name:?}")).into(),
            );
        }
    }

    Ok(())
}
