use std::{io, path::PathBuf};

use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    // wrappers of other libraries' errors
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("Failed to parse Tera template: {0}")]
    Tera(#[from] tera::Error),
    #[error("Failed to parse TOML: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("Failed to watch for file changes: {0}")]
    Notify(#[from] notify::Error),
    #[error("Development server error: {0}")]
    Rocket(#[from] rocket::Error),
    // favia-specific error
    #[error("Key not found in TOML frontmatter: {0}")]
    TomlLookup(String),
    #[error("Multiple factory templates found: {0}, {1}")]
    MultipleTemplateFactories(PathBuf, PathBuf),
    #[error("No factory template found for content file: {0}")]
    MissingTemplateFactory(PathBuf),
}
