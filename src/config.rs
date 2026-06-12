//! Configuration loaded from `.shaperc.toml`.
//!
//! Discovery walks up from the starting directory (typically `cwd`) until it
//! finds a `.shaperc.toml` or hits the filesystem root. A relative `dir` is
//! resolved against the directory containing the config file, so the same
//! config works regardless of which subdirectory shape is invoked from.

use crate::error::{Result, ShapeError};
use serde::Deserialize;
use std::path::{Path, PathBuf};

const FILENAME: &str = ".shaperc.toml";

/// Default workspace directory when nothing else is configured.
pub const DEFAULT_DIR: &str = ".shape";

#[derive(Debug, Deserialize, Default)]
struct ConfigFile {
    #[serde(default)]
    dir: Option<PathBuf>,
}

/// Effective shape workspace directory, given a CLI override and any
/// `.shaperc.toml` discovered walking up from `start`.
///
/// Precedence: explicit `cli_dir` > `.shaperc.toml` `dir` > default (`.shape`).
pub fn effective_dir(cli_dir: Option<&Path>, start: &Path) -> Result<PathBuf> {
    if let Some(dir) = cli_dir {
        return Ok(dir.to_path_buf());
    }
    if let Some(dir) = discover(start)? {
        return Ok(dir);
    }
    Ok(PathBuf::from(DEFAULT_DIR))
}

fn discover(start: &Path) -> Result<Option<PathBuf>> {
    for ancestor in start.ancestors() {
        let candidate = ancestor.join(FILENAME);
        if candidate.is_file() {
            let text = std::fs::read_to_string(&candidate)?;
            return parse(&text, ancestor).map_err(|message| ShapeError::Config {
                path: candidate.display().to_string(),
                message,
            });
        }
    }
    Ok(None)
}

fn parse(text: &str, base: &Path) -> std::result::Result<Option<PathBuf>, String> {
    let parsed: ConfigFile = toml::from_str(text).map_err(|err| err.to_string())?;
    Ok(parsed.dir.map(|dir| {
        if dir.is_absolute() {
            dir
        } else {
            base.join(dir)
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_dir_key_is_none() {
        assert_eq!(parse("", Path::new("/repo")).expect("ok"), None);
    }

    #[test]
    fn relative_dir_resolves_against_base() {
        assert_eq!(
            parse("dir = \".shape\"\n", Path::new("/repo")).expect("ok"),
            Some(PathBuf::from("/repo/.shape")),
        );
    }

    #[test]
    fn absolute_dir_is_kept_as_is() {
        assert_eq!(
            parse("dir = \"/abs/.shape\"\n", Path::new("/repo")).expect("ok"),
            Some(PathBuf::from("/abs/.shape")),
        );
    }

    #[test]
    fn malformed_toml_is_an_error() {
        assert!(parse("dir = [\n", Path::new("/repo")).is_err());
    }
}
