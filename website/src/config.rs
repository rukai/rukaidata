use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub legacy_renderer: bool,
    pub mods: Vec<String>,
    pub web_root: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = default_config_path();
        if path.exists() {
            serde_json::from_slice(&std::fs::read(path)?).map_err(|e| anyhow!(e))
        } else {
            let config = Config {
                legacy_renderer: false,
                mods: vec![],
                web_root: "/".to_owned(),
            };
            config.save(&path);
            Ok(config)
        }
    }

    fn save(&self, path: &Path) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, serde_json::to_vec_pretty(self).unwrap())
            .map_err(|e| anyhow::anyhow!("Failed to write to {path:?} {e}"))
            .unwrap();
    }
}

pub fn default_config_path() -> PathBuf {
    current_dir()
        .unwrap()
        .join("..")
        .join("data")
        .join("config.json")
}
