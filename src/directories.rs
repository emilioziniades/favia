use crate::error::Error;
use log::debug;
use std::{
    env,
    ffi::OsString,
    fs, io,
    path::{self, Path, PathBuf},
};

pub struct Directories {
    pub templates: path::PathBuf,
    pub content: path::PathBuf,
    pub build: path::PathBuf,
}

impl Directories {
    pub fn new() -> Result<Self, Error> {
        let cwd = env::current_dir()?;
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

    pub fn template_name(&self, content_path: &Path) -> OsString {
        // TODO: this assumes content and template files are flat directories
        // TODO: this assumes a 1-to-1 mapping of template to content
        content_path
            .with_extension("html")
            .file_name()
            .unwrap()
            .to_owned()
    }

    pub fn build_name(&self, template_name: &str) -> PathBuf {
        // TODO this assumes build folder is flat
        self.build.join(template_name)
    }
}
