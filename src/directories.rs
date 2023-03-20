use crate::{Error, Result};
use log::debug;
use std::{
    fs, io,
    path::{self, Path, PathBuf},
};
pub struct Directories {
    pub templates: path::PathBuf,
    pub content: path::PathBuf,
    pub build: path::PathBuf,
}

impl Directories {
    pub fn new(cwd: &Path) -> Result<Self> {
        debug!("working directory: {cwd:?}");

        let templates = cwd.join("templates");
        if !templates.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, format!("{templates:?}")).into());
        }
        debug!("templates directory: {templates:?}");
        let content = cwd.join("content");
        if !content.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, format!("{content:?}")).into());
        }
        debug!("content directory: {content:?}");

        let build = cwd.join(".favia");
        if build.exists() {
            debug!("cleaning build directory");
            fs::remove_dir_all(&build)?;
        }
        fs::create_dir(&build)?;
        debug!("build directory: {content:?}");

        Ok(Self {
            templates,
            content,
            build,
        })
    }

    pub fn find_template_path(&self, content_path: &Path) -> Result<PathBuf> {
        match self.direct_template_path(content_path) {
            Some(template_path) => {
                debug!("template file found: {template_path:#?}");
                Ok(template_path)
            }
            None => {
                debug!("no exact template file found. searching for factory template");
                self.factory_template_path(content_path)
            }
        }
    }

    pub fn factory_template_path(&self, content_path: &Path) -> Result<PathBuf> {
        let mut factory_template_path = None;
        let template_path_parent = self.templates.join(
            content_path
                .strip_prefix(&self.content)
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
                    return Err(Error::MultipleTemplateFactories(old_factory_template, path));
                }
                factory_template_path = Some(path);
            }
        }

        let factory_template_path = factory_template_path
            .ok_or_else(|| Error::MissingTemplateFactory(content_path.to_path_buf()))?;

        debug!("found factory template: {factory_template_path:#?}");
        Ok(factory_template_path)
    }

    // Provided with either content_file.md or content_file/index.md
    // Tries to find content_file.html or content_file/index.html
    // If neither exists, returns none
    pub fn direct_template_path(&self, content_path: &Path) -> Option<PathBuf> {
        // content_file/index.md
        if content_path.file_name().unwrap() == "index.md" {
            // content_file/index.html
            let template_path = self.templates.join(
                content_path
                    .strip_prefix(&self.content)
                    .unwrap()
                    .with_extension("html"),
            );
            if template_path.exists() {
                return Some(template_path);
            }

            // content_file.html
            let template_path = self.templates.join(
                content_path
                    .strip_prefix(&self.content)
                    .unwrap()
                    .parent()
                    .unwrap()
                    .with_extension("html"),
            );
            if template_path.exists() {
                return Some(template_path);
            }

            None
        }
        // content_file.md
        else {
            // content_file.html
            let template_path = self.templates.join(
                content_path
                    .strip_prefix(&self.content)
                    .unwrap()
                    .with_extension("html"),
            );
            if template_path.exists() {
                return Some(template_path);
            }

            // content_file/index.html
            let template_path = self.templates.join(
                content_path
                    .strip_prefix(&self.content)
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join(content_path.file_stem().unwrap())
                    .join("index.html"),
            );

            if template_path.exists() {
                return Some(template_path);
            }

            None
        }
    }

    pub fn make_build_path(&self, template_path: &Path, content_path: &Path) -> PathBuf {
        let file_name = template_path.file_name().unwrap().to_str().unwrap();

        if file_name.contains('[') && file_name.contains(']') {
            let slug = content_path.file_stem().unwrap();
            self.make_build_path(
                &template_path
                    .parent()
                    .unwrap()
                    .join(slug)
                    .with_extension("html"),
                content_path,
            )
        } else if file_name == "index.html" {
            self.build
                .join(template_path.strip_prefix(&self.templates).unwrap())
        } else {
            self.build.join(
                template_path
                    .strip_prefix(&self.templates)
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join(template_path.file_stem().unwrap())
                    .join("index.html"),
            )
        }
    }

    pub fn tera_template_name<'a>(&self, template_path: &'a Path) -> &'a str {
        template_path
            .strip_prefix(&self.templates)
            .unwrap()
            .to_str()
            .unwrap()
    }
}
