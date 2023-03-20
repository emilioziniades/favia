use std::{
    fs, io,
    path::{Path, PathBuf},
};

use log::{debug, trace};
use tera::Tera;

use crate::{directories::Directories, page_data::PageData, Result};

pub struct Builder {
    dirs: Directories,
    tera: Tera,
}

impl Builder {
    pub fn new(cwd: &Path) -> Result<Self> {
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
    ) -> Result<()> {
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

    pub fn template_path(&self, content_path: &Path) -> Result<PathBuf> {
        self.dirs.find_template_path(content_path)
    }

    pub fn build_path(&self, template_path: &Path, content_path: &Path) -> PathBuf {
        self.dirs.make_build_path(template_path, content_path)
    }

    pub fn tera_template_name(&self, template_path: &Path) -> Result<String> {
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

    pub fn build_content_file(&self, content_path: &Path) -> Result<()> {
        debug!("content file: {content_path:#?}");

        let template_path = self.template_path(content_path)?;

        let build_path = self.build_path(&template_path, content_path);
        debug!("build path: {build_path:?}");

        let tera_template_name = self.tera_template_name(&template_path)?;
        debug!("tera template found {tera_template_name:?}");

        let page_data: PageData = fs::read_to_string(content_path)?.try_into()?;
        trace!("parsed page data: {page_data:#?}");

        let context = tera::Context::from(page_data);
        trace!("tera context: {context:#?}");

        self.render(&tera_template_name, context, &build_path)?;

        Ok(())
    }
}
