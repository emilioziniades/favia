use crate::error::Error;
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
    pub fn new(cwd: PathBuf) -> Result<Self, Error> {
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

    // Provided with either content_file.md or content_file/index.md
    // Tries to find content_file.html or content_file/index.html
    // If neither exists, returns none
    pub fn template_path(&self, content_path: &Path) -> Option<PathBuf> {
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

            return None;
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

            return None;
        }
    }

    pub fn build_path(&self, template_path: Option<&PathBuf>) -> Option<PathBuf> {
        if let Some(template_path) = template_path {
            if template_path.file_name().unwrap() == "index.html" {
                Some(
                    self.build
                        .join(template_path.strip_prefix(&self.templates).unwrap()),
                )
            } else {
                Some(
                    self.build.join(
                        template_path
                            .strip_prefix(&self.templates)
                            .unwrap()
                            .parent()
                            .unwrap()
                            .join(template_path.file_stem().unwrap())
                            .join("index.html"),
                    ),
                )
            }
        } else {
            return None;
        }
        // self.build
        //     .join(template_path.strip_prefix(&self.templates).unwrap())
    }

    pub fn tera_template_name<'a>(&self, template_path: &'a Path) -> &'a str {
        template_path
            .strip_prefix(&self.templates)
            .unwrap()
            .to_str()
            .unwrap()
    }
}
