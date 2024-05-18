use std::path::PathBuf;

use oh_hai::config::Config;

/// Used by installation script to establish bookmark data file path prior to initial program run
fn main() -> Result<(), confy::ConfyError> {
    let path = std::env::args().skip(1).next();
    if let Some(p) = path {
        let config = Config {
            data_path: PathBuf::from(p)
        };
        confy::store("oh-hai", None, config)?;
    }
    Ok(())
}