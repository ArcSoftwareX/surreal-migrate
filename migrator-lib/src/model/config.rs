use crate::model::args::Args;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    pub database_url: String,
    pub pass: String,
    pub user: String,
    pub ns: String,
    pub db: String,
}

impl Config {
    pub fn new(args: &Args) -> anyhow::Result<Self> {
        let config = Self::read_config_file(&args.config_url)?;

        Ok(config)
    }

    // pub fn default() -> Self {
    //     Self {
    //         database_url: "http://127.0.0.1:8000".to_string(),
    //         db: "root".to_string(),
    //         ns: "root".to_string(),
    //         pass: "root".to_string(),
    //         user: "root".to_string(),
    //     }
    // }

    fn read_config_file(config_url: &String) -> anyhow::Result<Config> {
        let exists = Path::new(config_url).exists();

        if !exists {
            anyhow::bail!("Config file does not exist: {config_url}")
        }

        let contents = fs::read_to_string(config_url)?;

        Ok(serde_json::from_str(&contents)?)
    }
}
