use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use serde_derive::Deserialize;
use anyhow::Result;
use once_cell::sync::Lazy;
use crate::cli::CONFIG_FILE;
use tokio::fs;
use parking_lot::Mutex;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub comm: Comm,
}

#[derive(Debug, Default, Deserialize)]
pub struct Comm {
    #[serde(default = "default_current_path")]
    download_path: String,
    #[serde(default = "default_cpus_num")]
    subprocess_num: u64,
}

fn default_current_path() -> String {
    env::current_dir().unwrap_or(PathBuf::from("./")).as_os_str().to_str().unwrap().to_string()
}

fn default_cpus_num() -> u64 {
    num_cpus::get() as u64 * 4
}

pub static CFG: Lazy<Arc<Mutex<Config>>> = Lazy::new(|| Arc::new(Mutex::new(Default::default())));

impl Config {
    pub async fn new<P: AsRef<Path>>(f: P) -> Result<Self> {
        if f.as_ref() == Path::new(CONFIG_FILE) {
            return Ok(toml::from_slice(
                &fs::read(std::env::current_dir()?.join(f)).await?,
            )?);
        }

        Ok(toml::from_slice(&fs::read(f).await?)?)
    }
}

#[cfg(test)]
mod tests {
    use tracing::info;
    use super::*;

    #[tokio::test]
    async fn test_config_new_should_work() {
        tracing_subscriber::fmt().init();
        let config = Config::new(Path::new(CONFIG_FILE)).await.unwrap();
        info!("{:#?}", config);
    }
}