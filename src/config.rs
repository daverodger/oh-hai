use std::path::PathBuf;

use home;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub data_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let mut default_path = home::home_dir().unwrap();
        default_path.push("cli/oh_hai/data/bookmarks.json");
        Self { data_path: default_path }
    }
}

pub fn get_config_file() -> Config {
    let config: Config = confy::load("oh-hai", None).unwrap();
    config
}

pub fn get_data_file_path() -> PathBuf {
    get_config_file().data_path
}