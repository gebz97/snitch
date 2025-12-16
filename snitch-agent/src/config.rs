use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use serde_yaml::from_str;
use std::path;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgentConfiguration {
    aggregator_host: String,
    aggregator_port: i16,
    max_retries: i16,
    log_level: String
}

pub fn load_config(config: String) -> Result<AgentConfiguration> {
    let config_file_contents = fs::read_to_string(config)?;
    println!("Contents:\n{}", config_file_contents);
    let agent_config :AgentConfiguration = from_str(&config_file_contents)?;
    Ok(agent_config)
}