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

    pub fn template_path(&self, content_path: &Path) -> PathBuf {
        // TODO: this assumes a 1-to-1 mapping of template to content
        self.templates.join(
            content_path
                .strip_prefix(&self.content)
                .unwrap()
                .with_extension("html"),
        )
    }

    pub fn build_path<'a>(&self, template_path: &'a Path) -> PathBuf {
        self.build
            .join(template_path.strip_prefix(&self.templates).unwrap())
    }

    pub fn tera_template_name<'a>(&self, template_path: &'a Path) -> &'a str {
        template_path
            .strip_prefix(&self.templates)
            .unwrap()
            .to_str()
            .unwrap()
    }
}
