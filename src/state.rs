use log::warn;
use serde::Deserialize;
use std::{fs, path::PathBuf};

use crate::error::{ClusteritError, ClusteritErrorKind};

#[derive(Deserialize, Debug)]
pub struct Device {
    pub destination: String,
    pub ssh_port: u16,
    pub ssh_key: String,
}

#[derive(Deserialize, Debug)]
pub struct State {
    pub devices: Vec<Device>,
}

impl State {
    pub fn from_file(path: &PathBuf) -> Result<State, ClusteritError> {
        if !path.is_file() {
            warn!("State file not found, loading empty state file");
            return Ok(State::default());
        }

        let content = fs::read_to_string(path).or(Err(ClusteritError::new(
            ClusteritErrorKind::ParseError,
            format!("Could not read state from: {path:?}"),
        )))?;

        let state: State = serde_json::from_str(&content)?;

        return Ok(state);
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            devices: Default::default(),
        }
    }
}
