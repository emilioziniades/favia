use std::{
    fs, io,
    path::{Path, PathBuf},
};

use tera::Tera;

use crate::{directories::Directories, error::Error};

pub struct Builder {
    dirs: Directories,
    tera: Tera,
}

impl Builder {
    pub fn new(cwd: &Path) -> Result<Self, Error> {
        let dirs = Directories::new(cwd)?;

        Ok(Self {
            tera: Tera::new(dirs.templates.join("**").join("*").to_str().unwrap())?,
            dirs,
        })
    }

    pub fn render(
        &self,
        template_name: &str,
        context: tera::Context,
        build_path: &Path,
    ) -> Result<(), Error> {
        fs::create_dir_all(build_path.parent().unwrap())?;
        let build_file = fs::File::create(build_path)?;
        self.tera.render_to(template_name, &context, build_file)?;
        Ok(())
    }

    pub fn templates_folder(&self) -> &Path {
        &self.dirs.templates
    }

    pub fn content_folder(&self) -> &Path {
        &self.dirs.content
    }

    pub fn build_folder(&self) -> PathBuf {
        self.dirs.build.clone()
    }

    pub fn template_path(&self, content_path: &Path) -> Result<PathBuf, Error> {
        self.dirs.find_template_path(content_path)
    }

    pub fn build_path(&self, template_path: &Path, content_path: &Path) -> PathBuf {
        self.dirs.make_build_path(template_path, content_path)
    }

    pub fn tera_template_name(&self, template_path: &Path) -> Result<String, Error> {
        let tera_template_name = self.dirs.tera_template_name(template_path);
        match self
            .tera
            .get_template_names()
            .find(|name| name == &tera_template_name)
        {
            Some(tera_template_name) => Ok(tera_template_name.to_string()),
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("{tera_template_name:?}"),
            ))?,
        }
    }
}
