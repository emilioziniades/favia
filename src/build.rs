use crate::Result;
use crate::{builder::Builder, directories::Directories};
use log::{debug, info};
use std::path::Path;
use walkdir::WalkDir;

pub fn build(cwd: &Path) -> Result<()> {
    info!("building site");
    info!("preparing directories");
    let builder = Builder::new(cwd)?;
    let dirs = Directories::new(cwd)?;

    info!("building files");
    for entry in WalkDir::new(&dirs.content)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
    {
        let content_path = entry.path();
        debug!("content file: {content_path:#?}");

        builder.build_content_file(content_path)?;
    }

    Ok(())
}
