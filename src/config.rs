use std::path::PathBuf;

use home;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub data_path: PathBuf,
    pub output_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let home_path = home::home_dir().unwrap();
        let mut dp = home_path.clone();
        dp.push("cli/oh_hai/data/bookmarks.json");
        let mut op = home_path.clone();
        op.push("cli/oh_hai/data/.command.txt");
        Self {
            data_path: dp,
            output_path: op,
        }
    }
}

// Creates config file if not found and returns Config struct
fn get_config_file() -> Config {
    let config: Config = confy::load("oh-hai", None).unwrap();
    config
}

pub fn get_data_file_path() -> PathBuf {
    get_config_file().data_path
}

pub fn get_output_file_path() -> PathBuf {
    get_config_file().output_path
}
