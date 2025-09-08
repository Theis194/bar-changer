use std::fs::{create_dir, exists};

use crate::BarChangerError;
use crate::error::errors::Context;
use crate::files::{files::file_exists, get_home_dir, read_file, write_file};
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, BarChangerError>;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub home_dir: String,
    pub waybar_dir: String,
}

impl Config {
    pub fn use_default_dirs() -> Self {
        Self {
            home_dir: get_home_dir().expect("Failed to get users home dir"),
            waybar_dir: ".config/waybar/".to_string(),
        }
    }

    pub fn write(&self) -> Result<()> {
        let serialized_config = toml::to_string(self)
            .map_err(|_| BarChangerError::Serialization("config".to_string()))?;

        let config_location = format!("{}/.config/bar-changer/config.toml", self.home_dir);

        write_file(&config_location, serialized_config)
            .context("Failed to write config".to_string())?;

        Ok(())
    }

    pub fn load() -> Result<Config> {
        let home_dir = get_home_dir()?;
        let config_location = format!("{}/.config/bar-changer/config.toml", home_dir);

        if file_exists(&config_location)? {
            let config_raw = read_file(&config_location)?;
            let config: Config = toml::from_str(&config_raw)
                .map_err(|_| BarChangerError::DeSerialization("config".to_string()))?;
            return Ok(config);
        }

        Err(BarChangerError::ConfigNotFound(
            "Config file did not exist".to_string(),
        ))
    }

    pub fn create_dir(&self) -> Result<()> {
        let config_dir = format!("{}/.config/bar-changer/", self.home_dir);
        if !exists(&config_dir)? {
            create_dir(config_dir)
                .map_err(|e| BarChangerError::Io(e))
                .context("Failed to create bar-changer config dir")?;
        }

        Ok(())
    }

    pub fn dir_exists(&self) -> Result<bool> {
        let config_dir = format!("{}/.config/bar-changer/", self.home_dir);

        exists(config_dir)
            .map_err(|e| BarChangerError::Io(e))
            .context("Failed to find config dir")
    }
}

#[derive(Deserialize, Serialize)]
pub struct Cache {
    pub last_bar: Option<String>,
    pub last_style: Option<String>,
}

impl Cache {
    pub fn write(&self) -> Result<()> {
        let serialized_cache = toml::to_string(self)
            .map_err(|_| BarChangerError::DeSerialization("cache".to_string()))
            .context("Failed to serialize cache")?;

        let home_dir = get_home_dir()?;
        let cache_location = format!("{}/.cache/bar-changer/cache.toml", home_dir);

        write_file(&cache_location, serialized_cache)?;

        Ok(())
    }

    pub fn load() -> Result<Cache> {
        let home_dir = get_home_dir().expect("Failed to get home dir");
        let cache_location = format!("{}/.cache/bar-changer/cache.toml", home_dir);

        if file_exists(&cache_location)? {
            let cache_raw = read_file(&cache_location).context("Failed to read cache file")?;
            let cache: Cache = toml::from_str(&cache_raw)
                .map_err(|_| BarChangerError::DeSerialization("cache".to_string()))
                .context("Failed deserializing cache")?;

            return Ok(cache);
        } else {
            let cache_dir = format!("{}/.cache/bar-changer/", home_dir);
            if !exists(&cache_dir)? {
                create_dir(cache_dir)
                    .map_err(|e| BarChangerError::Io(e))
                    .context("Failed to create bar-changer cache dir")?;
            }
            let cache = Cache {
                last_bar: None,
                last_style: None,
            };

            write_file(
                cache_location.as_str(),
                toml::to_string(&cache).expect("Failed to serialize cache"),
            )
            .context("Failed to create cache file")?;
            return Ok(cache);
        }
    }
}
