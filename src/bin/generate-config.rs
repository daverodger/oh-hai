use std::path::PathBuf;

use oh_hai::config::Config;

/// Used by installation script to establish bookmark data file path prior to initial program run
fn main() -> Result<(), confy::ConfyError> {
    let path = PathBuf::from(std::env::args().skip(1).next().unwrap());
    let mut dp = path.clone();
    dp.push("../../bookmarks.json");
    let mut op = path.clone();
    op.push("../../.command.txt");
    let config = Config {
        data_path: dp,
        output_path: op,
    };
    confy::store("oh-hai", None, config)?;
    Ok(())
}
