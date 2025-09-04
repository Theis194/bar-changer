use std::fs::{create_dir, exists};

use crate::files::{files::file_exists, get_home_dir, read_file, write_file};
use serde::{Deserialize, Serialize};

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
}

#[derive(Deserialize, Serialize)]
pub struct Cache {
    pub last_bar: Option<String>,
    pub last_style: Option<String>,
}

impl Cache {
    pub fn write(&self) -> Result<(), Box<dyn std::error::Error>> {
        let serialized_cache = toml::to_string(self).expect("Failed to serialize cache");

        let home_dir = get_home_dir().expect("Failed to get home dir");
        let config_location = format!("{}/.cache/bar-changer/cache.toml", home_dir);

        write_file(&config_location, serialized_cache).expect("Failed to write cache to file");

        Ok(())
    }
}

pub fn read_cache() -> Result<Cache, Box<dyn std::error::Error>> {
    let home_dir = get_home_dir().expect("Failed to get home dir");
    let cache_location = format!("{}/.cache/bar-changer/cache.toml", home_dir);
    if file_exists(&cache_location) {
        let cache_raw = read_file(&cache_location).expect("Failed to read cache file");
        let cache: Cache = toml::from_str(&cache_raw).expect("Failed deserializing cache");
        return Ok(cache);
    } else {
        let cache_dir = format!("{}/.cache/bar-changer/", home_dir);
        if !exists(&cache_dir)? {
            create_dir(cache_dir).expect("Failed to create bar-changer cache dir");
        }
        let cache = Cache {
            last_bar: None,
            last_style: None,
        };

        write_file(
            cache_location.as_str(),
            toml::to_string(&cache).expect("Failed to serialize cache"),
        )
        .expect("Failed to create cache file");
        return Ok(cache);
    }
}

pub fn read_config() -> Option<Config> {
    let home_dir = get_home_dir().expect("Failed to get home dir");
    let config_location = format!("{}/.cache/bar-changer/cache.toml", home_dir);
    if file_exists(&config_location) {
        let config_raw = read_file(&config_location).expect("Failed to read cache file");
        let config: Config = toml::from_str(&config_raw).expect("Failed deserializing cache");
        return Some(config);
    }
    None
}
