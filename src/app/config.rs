// ********************* import ********************* //
use config::{Config, Environment, File};
use serde::Deserialize;

use super::common::prelude::*;
use crate::app::{
    db::DBConfig,
    utils::{cache::CacheConfig, log::LogConfig},
};

// ********************* content ********************* //
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct ServiceConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            host: "localhost".into(),
            port: 8069,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub cache: CacheConfig,
    #[serde(default)]
    pub db: DBConfig,
    #[serde(default)]
    pub log: LogConfig,
    #[serde(default)]
    pub service: ServiceConfig,
}

impl AppConfig {
    pub fn init(cfg_file_path: &str) -> AppResult<Self> {
        Config::builder()
            .add_source(File::with_name(cfg_file_path))
            .add_source(Environment::with_prefix("SPACE_BACKEND").separator("__"))
            .build()
            .wrap_with(
                || format!("Failed to build 'Config' from sources: {}", cfg_file_path),
                AppErrorKind::ConfigurationError,
            )?
            .try_deserialize()
            .wrap(
                "Failed to deserialize 'Config' into 'AppConfig'",
                AppErrorKind::ConfigurationError,
            )
    }
}
