use crate::domain::error::{DomainError, DomainResult};
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
    #[serde(default)]
    pub app_id: Option<String>,
    #[serde(default)]
    pub app_secret: Option<String>,
}

impl AuthConfig {
    pub fn has_app_credentials(&self) -> bool {
        self.app_id.is_some() && self.app_secret.is_some()
    }
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

fn config_path() -> DomainResult<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| DomainError::Config("Cannot find home directory".to_string()))?;
    Ok(home.join(CONFIG_DIR).join(CONFIG_FILE))
}

pub fn load() -> DomainResult<Config> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(Config::default());
    }
    let content = fs::read_to_string(&path)
        .map_err(|e| DomainError::Io(format!("Failed to read config: {}", e)))?;
    let config: Config = toml::from_str(&content)
        .map_err(|e| DomainError::Config(format!("Failed to parse config: {}", e)))?;
    Ok(config)
}

pub fn save(config: &Config) -> DomainResult<()> {
    let path = config_path()?;
    let dir = path
        .parent()
        .ok_or_else(|| DomainError::Config("Invalid config path".to_string()))?;
    fs::create_dir_all(dir)
        .map_err(|e| DomainError::Io(format!("Failed to create config dir: {}", e)))?;
    let content = toml::to_string_pretty(config)
        .map_err(|e| DomainError::Config(format!("Failed to serialize config: {}", e)))?;
    fs::write(&path, content)
        .map_err(|e| DomainError::Io(format!("Failed to write config: {}", e)))?;
    Ok(())
}
