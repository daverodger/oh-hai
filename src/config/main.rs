use std::path::PathBuf;

use oh_hai::config::Config;

/// Used by installation script to establish bookmark data file path prior to initial program run
fn main() {
    let path = std::env::args().skip(1).next();
    if let Some(p) = path {
        let config = Config {
            data_path: PathBuf::from(p)
        };
        let _ = confy::store("oh-hai", None, config);
    }
}