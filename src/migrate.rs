use std::path::Path;
use std::error::Error as StdError;

use crate::utility;

pub fn migrate(migrate_file: &str) -> Result<(), Box<dyn StdError>> {
    let migrate_file_path = Path::new(migrate_file).canonicalize()?;
    utility::check_file(&migrate_file_path)?;

    Ok(())
}
