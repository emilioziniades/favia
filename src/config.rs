use std::{fs, path::Path};

use crate::error;
use log::{info, trace};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub version: String,
    pub base_url: String,
}

impl TryFrom<&Path> for Config {
    type Error = error::Error;

    fn try_from(config_path: &Path) -> Result<Self, Self::Error> {
        let config_contents = fs::read_to_string(config_path)?;
        Self::try_from(config_contents.as_str())
    }
}

impl TryFrom<&str> for Config {
    type Error = error::Error;

    fn try_from(config_contents: &str) -> Result<Self, Self::Error> {
        let config: Config = toml::from_str(config_contents)?;
        info!("parsing config");
        trace!("config parsed:\n{config:#?}");
        Ok(config)
    }
}

mod tests {

    #[test]
    fn parses_config() {
        let config = "
version = \"0.0.1\"
base_url = \"https://example.com\"
";
        let config = crate::config::Config::try_from(config).expect("valid config");

        assert_eq!(config.version, "0.0.1");
        assert_eq!(config.base_url, "https://example.com")
    }

    #[test]
    fn errors_on_bad_config() {
        let config = "
vershun = \"0.0.1\"
baSE_Url= \"https://example.com\"
";
        crate::config::Config::try_from(config).unwrap_err();
    }
}
