use crate::error::{CliError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const CONFIG_DIR: &str = ".skillhub";
const CONFIG_FILE: &str = "config.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub registry: RegistryConfig,
    #[serde(default)]
    pub auth: AuthConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RegistryConfig {
    #[serde(default = "default_registry_url")]
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AuthConfig {
    #[serde(default)]
    pub token: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            registry: RegistryConfig {
                url: default_registry_url(),
            },
            auth: AuthConfig::default(),
        }
    }
}

fn default_registry_url() -> String {
    String::from("http://localhost:3001")
}

fn config_path() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| CliError::Config("Cannot find home directory".to_string()))?;
    Ok(home.join(CONFIG_DIR).join(CONFIG_FILE))
}

pub fn load() -> Result<Config> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(Config::default());
    }
    let content = fs::read_to_string(&path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

pub fn save(config: &Config) -> Result<()> {
    let path = config_path()?;
    let dir = path.parent().ok_or_else(|| CliError::Config("Invalid config path".to_string()))?;
    fs::create_dir_all(dir)?;
    let content = toml::to_string_pretty(config)?;
    fs::write(&path, content)?;
    Ok(())
}
