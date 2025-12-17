use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_yaml::from_str;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgentConfiguration {
    aggregator_host: String,
    aggregator_port: i16,
    max_retries: i16,
    pid_file: PathBuf,

    #[serde(default)]
    log: LogConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogConfig {
    #[serde(default = "default_log_level")]
    level: LogLevel,
    
    #[serde(default)]
    location: LogLocation,
}

fn default_log_level() -> LogLevel {
    LogLevel::Info
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            location: LogLocation::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum LogLocation {
    Stdout,
    File {
        #[serde(default = "default_log_path")]
        path: PathBuf,
    },
}

fn default_log_path() -> PathBuf {
    PathBuf::from("/var/log/snitch.log")
}

impl Default for LogLocation {
    fn default() -> Self {
        LogLocation::File {
            path: default_log_path(),
        }
    }
}

impl LogConfig {
    /// Get the log level
    #[allow(dead_code)]
    pub fn level(&self) -> &LogLevel {
        &self.level
    }
    
    /// Check if logging to stdout
    #[allow(dead_code)]
    pub fn is_stdout(&self) -> bool {
        matches!(self.location, LogLocation::Stdout)
    }
    
    /// Get the file path if logging to file, None if logging to stdout
    #[allow(dead_code)]
    pub fn file_path(&self) -> Option<&PathBuf> {
        match &self.location {
            LogLocation::File { path } => Some(path),
            LogLocation::Stdout => None,
        }
    }
    
    /// Get a reference to the log location
    #[allow(dead_code)]
    pub fn location(&self) -> &LogLocation {
        &self.location
    }
}

pub fn load_config(config: String) -> Result<AgentConfiguration> {
    let config_file_contents = fs::read_to_string(config)?;
    let agent_config: AgentConfiguration = from_str(&config_file_contents)?;
    
    // Type system guarantees valid state
    Ok(agent_config)
}

