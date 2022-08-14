use std::{collections::HashMap, fs, path::Path, path::PathBuf};

use anyhow::Result;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub servers: HashMap<String, Server>,
    pub theme: String,
    #[serde(skip)]
    path: Option<PathBuf>,
}

impl Config {
    pub fn load<T: AsRef<Path>>(path: T) -> Result<Self> {
        let mut me = match fs::read_to_string(&path) {
            Ok(raw) => ron::from_str(&raw).unwrap_or_default(),
            Err(_) => Self::default(),
        };

        me.path = Some(path.as_ref().to_path_buf());

        Ok(me)
    }

    pub fn save(&self) -> Result<()> {
        let s = ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::new())?;
        fs::write(
            self.path
                .as_ref()
                .expect("Tried to save config with no path"),
            &s,
        )?;

        Ok(())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Server {
    pub url: String,
    pub port: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Msg {
    Start { target: Server, password: String },
    Send { message: String },
    Stop,
}
