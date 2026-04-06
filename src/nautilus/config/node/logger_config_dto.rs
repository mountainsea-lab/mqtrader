use ahash::AHashMap;
use anyhow::Result;
use log::LevelFilter;
use nautilus_common::logging::config::LoggerConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ustr::Ustr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggerConfigDTO {
    pub stdout_level: String,
    pub fileout_level: String,

    pub component_level: HashMap<String, String>,
    pub module_level: HashMap<String, String>,

    pub log_components_only: bool,
    pub is_colored: bool,
    pub print_config: bool,
    pub use_tracing: bool,
}

fn parse_level(level: &str) -> Result<LevelFilter> {
    match level.to_lowercase().as_str() {
        "off" => Ok(LevelFilter::Off),
        "error" => Ok(LevelFilter::Error),
        "warn" => Ok(LevelFilter::Warn),
        "info" => Ok(LevelFilter::Info),
        "debug" => Ok(LevelFilter::Debug),
        "trace" => Ok(LevelFilter::Trace),
        _ => anyhow::bail!("Invalid log level: {}", level),
    }
}

fn level_to_string(level: LevelFilter) -> String {
    format!("{:?}", level)
}

//DTO → LoggerConfig
impl TryFrom<LoggerConfigDTO> for LoggerConfig {
    type Error = anyhow::Error;

    fn try_from(dto: LoggerConfigDTO) -> Result<Self> {
        let component_level = dto
            .component_level
            .into_iter()
            .map(|(k, v)| Ok((Ustr::from(&k), parse_level(&v)?)))
            .collect::<Result<AHashMap<_, _>>>()?;

        let module_level = dto
            .module_level
            .into_iter()
            .map(|(k, v)| Ok((Ustr::from(&k), parse_level(&v)?)))
            .collect::<Result<AHashMap<_, _>>>()?;

        Ok(Self {
            stdout_level: parse_level(&dto.stdout_level)?,
            fileout_level: parse_level(&dto.fileout_level)?,
            component_level,
            module_level,
            log_components_only: dto.log_components_only,
            is_colored: dto.is_colored,
            print_config: dto.print_config,
            use_tracing: dto.use_tracing,
        })
    }
}

// LoggerConfig → DTO
impl From<&LoggerConfig> for LoggerConfigDTO {
    fn from(cfg: &LoggerConfig) -> Self {
        let component_level = cfg
            .component_level
            .iter()
            .map(|(k, v)| (k.to_string(), level_to_string(*v)))
            .collect();

        let module_level = cfg
            .module_level
            .iter()
            .map(|(k, v)| (k.to_string(), level_to_string(*v)))
            .collect();

        Self {
            stdout_level: level_to_string(cfg.stdout_level),
            fileout_level: level_to_string(cfg.fileout_level),
            component_level,
            module_level,
            log_components_only: cfg.log_components_only,
            is_colored: cfg.is_colored,
            print_config: cfg.print_config,
            use_tracing: cfg.use_tracing,
        }
    }
}
