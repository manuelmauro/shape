//! Scaffold a shape workspace: the `.shape/` directory tree and `.shaperc.toml`.

use crate::cli::Cli;
use crate::config;
use crate::error::{Result, ShapeError};
use colored::Colorize;
use std::path::{Path, PathBuf};

/// Workspace sub-directories, one per kind of artifact in the Shape Up loop.
const SUBDIRS: [&str; 4] = ["pitches", "bets", "scopes", "hill"];

const WORKSPACE_README: &str = "\
# shape workspace

Artifacts of the Shape Up loop (shape → bet → build → ship).

- `pitches/` — shaped pitches: problem · appetite · solution · rabbit holes · no-gos
- `bets/`    — placed bets: a pitch committed to a run, with the armed budget
- `scopes/`  — independently completable slices, with tasks and hill position
- `hill/`    — append-only hill-chart snapshots (how the work is moving)

See `docs/PRODUCT.md` for the full method.
";

/// Run the `init` command.
pub fn run(cli: &Cli) -> Result<i32> {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let dir = cli
        .dir
        .clone()
        .unwrap_or_else(|| PathBuf::from(config::DEFAULT_DIR));

    let workspace = scaffold(&cwd, &dir)?;

    if !cli.quiet {
        println!(
            "{} Initialized shape workspace at {}",
            "✓".green().bold(),
            workspace.display()
        );
        println!("  • wrote .shaperc.toml");
        for sub in SUBDIRS {
            println!("  • {}/", workspace.join(sub).display());
        }
        println!(
            "\nNext: {} to start co-shaping a pitch.",
            "shape pitch new <name>".bold()
        );
    }
    Ok(0)
}

/// Create the workspace tree and `.shaperc.toml` under `base`, returning the
/// resolved workspace path. Errors if a `.shaperc.toml` already exists.
fn scaffold(base: &Path, dir: &Path) -> Result<PathBuf> {
    let rc_path = base.join(".shaperc.toml");
    if rc_path.exists() {
        return Err(ShapeError::AlreadyInitialized {
            path: base.display().to_string(),
        });
    }

    let workspace = if dir.is_absolute() {
        dir.to_path_buf()
    } else {
        base.join(dir)
    };

    for sub in SUBDIRS {
        let path = workspace.join(sub);
        std::fs::create_dir_all(&path)?;
        std::fs::write(path.join(".gitkeep"), "")?;
    }
    std::fs::write(workspace.join("README.md"), WORKSPACE_README)?;
    std::fs::write(&rc_path, rc_contents(dir))?;

    Ok(workspace)
}

fn rc_contents(dir: &Path) -> String {
    format!(
        "# shape workspace configuration.\n\
         # `dir` is the workspace directory, resolved relative to this file.\n\
         dir = \"{}\"\n",
        dir.display(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn scaffold_creates_workspace_and_config() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let workspace = scaffold(tmp.path(), Path::new(".shape")).expect("scaffold");

        assert_eq!(workspace, tmp.path().join(".shape"));
        assert!(tmp.path().join(".shaperc.toml").is_file());
        assert!(workspace.join("README.md").is_file());
        for sub in SUBDIRS {
            assert!(workspace.join(sub).is_dir(), "missing {sub}/");
        }
    }

    #[test]
    fn scaffold_errors_when_already_initialized() {
        let tmp = tempfile::tempdir().expect("tempdir");
        fs::write(tmp.path().join(".shaperc.toml"), "dir = \".shape\"\n").expect("write");

        let err = scaffold(tmp.path(), Path::new(".shape")).expect_err("should error");
        assert!(matches!(err, ShapeError::AlreadyInitialized { .. }));
    }
}
