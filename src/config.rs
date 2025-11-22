use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub installation_dir: PathBuf,
    pub active_version: Option<String>,
    pub installed_versions: Vec<JavaVersion>,
    pub last_scan: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaVersion {
    pub version: String,
    pub vendor: String,
    pub path: PathBuf,
    pub is_lts: bool,
    pub architecture: String,
    pub last_used: Option<DateTime<Utc>>,
    pub auto_detected: bool,
}

impl Config {
    pub fn new(installation_dir: PathBuf) -> Self {
        Self {
            installation_dir,
            active_version: None,
            installed_versions: Vec::new(),
            last_scan: None,
        }
    }

    pub fn config_dir() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?
            .join("jaman");

        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }

        Ok(config_dir)
    }

    pub fn config_file() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    pub fn load() -> Result<Self> {
        let config_file = Self::config_file()?;

        if !config_file.exists() {
            return Self::default_config();
        }

        let content = fs::read_to_string(config_file)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_file = Self::config_file()?;
        let content = toml::to_string_pretty(self)?;
        fs::write(config_file, content)?;
        Ok(())
    }

    fn default_config() -> Result<Self> {
        let installation_dir = dirs::data_local_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))?
            .join("jaman")
            .join("jdks");

        if !installation_dir.exists() {
            fs::create_dir_all(&installation_dir)?;
        }

        Ok(Self::new(installation_dir))
    }

    pub fn add_version(&mut self, version: JavaVersion) {
        // Remove if exists
        self.installed_versions.retain(|v| v.path != version.path);
        self.installed_versions.push(version);
    }

    pub fn remove_version(&mut self, path: &PathBuf) {
        self.installed_versions.retain(|v| &v.path != path);
    }

    pub fn get_version(&self, version_str: &str) -> Option<&JavaVersion> {
        self.installed_versions
            .iter()
            .find(|v| v.version.contains(version_str))
    }

    pub fn set_active(&mut self, version: &str) -> Result<()> {
        if self.get_version(version).is_some() {
            self.active_version = Some(version.to_string());
            Ok(())
        } else {
            Err(anyhow::anyhow!("Version {} not found", version))
        }
    }
}

impl JavaVersion {
    pub fn new(
        version: String,
        vendor: String,
        path: PathBuf,
        is_lts: bool,
        architecture: String,
        auto_detected: bool,
    ) -> Self {
        Self {
            version,
            vendor,
            path,
            is_lts,
            architecture,
            last_used: None,
            auto_detected,
        }
    }

    pub fn java_executable(&self) -> PathBuf {
        if cfg!(windows) {
            self.path.join("bin").join("java.exe")
        } else {
            self.path.join("bin").join("java")
        }
    }

    #[allow(dead_code)]
    pub fn javac_executable(&self) -> PathBuf {
        if cfg!(windows) {
            self.path.join("bin").join("javac.exe")
        } else {
            self.path.join("bin").join("javac")
        }
    }

    pub fn mark_used(&mut self) {
        self.last_used = Some(Utc::now());
    }
}
