use crate::builder::Builder;
use crate::Result;
use log::info;
use std::path::Path;

pub fn build(cwd: &Path) -> Result<()> {
    info!("building site");
    info!("preparing directories");
    let builder = Builder::new(cwd)?;

    info!("building files");
    builder.build()?;

    Ok(())
}
