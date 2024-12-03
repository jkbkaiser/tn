use miette::{miette, IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub src: PathBuf,
    pub assets: Option<PathBuf>,
}

impl Config {
    pub fn parse<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = fs::read_to_string(&path)
            .map_err(|err| miette!("Could not parse config file: {err}"))?;
        let mut config: Config = toml::from_str(&contents)
            .map_err(|err| miette!("Could not parse config file: {err}"))?;

        let mut absolute_config_dir = path.as_ref().canonicalize().into_diagnostic()?;
        absolute_config_dir.pop();

        let relative_src = config.src.components().last().unwrap();
        config.src = absolute_config_dir.join(relative_src);

        if config.assets.is_none() {
            let assets_dir = crate::get_assets_dir();
            config.assets = Some(assets_dir);
        }

        Ok(config)
    }
}
