use std::path::PathBuf;

use oh_hai::config::Config;

/// Used by installation script to establish bookmark data file path prior to initial program run
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args()
        .nth(1)
        .ok_or("Configuration path required as first argument")?;
    let mut data_path = PathBuf::from(path);
    data_path.push("bookmarks.json");
    let config = Config { data_path };
    confy::store("oh-hai", None, config)?;

    Ok(())
}
