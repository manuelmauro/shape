//! Shared access to the `.shape/` workspace: locating artifacts and
//! reading/writing their YAML frontmatter.
//!
//! Documents are `---\n<yaml>\n---\n<body>`. We parse and rewrite only the
//! frontmatter, preserving the body verbatim, so human/agent edits to the
//! Markdown survive CLI state transitions.

use crate::cli::{Appetite, Cli};
use crate::config;
use crate::error::{Result, ShapeError};
use crate::slug::slugify;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};

/// Resolve the workspace directory for this invocation (CLI flag, then
/// `.shaperc.toml` discovery, then the `.shape` default).
pub fn resolve(cli: &Cli) -> Result<PathBuf> {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    config::effective_dir(cli.dir.as_deref(), &cwd)
}

/// Pitch frontmatter.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Pitch {
    /// Pitch slug (matches the filename stem).
    pub name: String,
    /// Human-readable title.
    pub title: String,
    /// Appetite: `small` or `big`.
    pub appetite: String,
    /// Lifecycle status: `shaping` or `shaped`.
    pub status: String,
    /// Creation date (ISO `YYYY-MM-DD`).
    pub created: String,
}

/// Bet frontmatter.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Bet {
    /// Zero-padded bet id (e.g. `001`).
    pub id: String,
    /// Pitch slug this bet commits to.
    pub pitch: String,
    /// Appetite carried over from the pitch.
    pub appetite: String,
    /// Run status: `active`, `building`, `shipped`, or `stopped`.
    pub status: String,
    /// Date the bet was placed (ISO `YYYY-MM-DD`).
    pub placed: String,
    /// Armed circuit-breaker budget.
    pub budget: Budget,
    /// Whether the first-slice gate has been accepted.
    #[serde(default)]
    pub slice_accepted: bool,
}

/// The armed circuit-breaker budget for a bet.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Budget {
    /// Token ceiling.
    pub tokens: u64,
    /// Iteration ceiling.
    pub iterations: u32,
    /// Wall-clock ceiling, in minutes.
    pub minutes: u32,
}

impl Budget {
    /// Default budget caps for an appetite.
    pub fn for_appetite(appetite: Appetite) -> Budget {
        match appetite {
            Appetite::Small => Budget {
                tokens: 60_000,
                iterations: 40,
                minutes: 20,
            },
            Appetite::Big => Budget {
                tokens: 200_000,
                iterations: 120,
                minutes: 90,
            },
        }
    }
}

/// Scope frontmatter.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Scope {
    /// Scope slug (matches the filename stem).
    pub name: String,
    /// Bet id this scope belongs to.
    pub bet: String,
    /// Status: `todo` or `done`.
    pub status: String,
    /// Hill position, 0 (start, uphill) to 100 (done, downhill).
    pub hill: u8,
}

const FENCE: &str = "\n---\n";

/// Split `---\n<yaml>\n---\n<body>` into its `(yaml, body)` parts.
pub fn split_frontmatter(text: &str) -> Option<(&str, &str)> {
    let rest = text.strip_prefix("---\n")?;
    let idx = rest.find(FENCE)?;
    Some((&rest[..idx], &rest[idx + FENCE.len()..]))
}

/// Compose a document from a serializable frontmatter value and a body.
pub fn compose<T: Serialize>(frontmatter: &T, body: &str) -> String {
    let yaml = serde_yaml::to_string(frontmatter)
        .expect("frontmatter serialization is infallible for these types");
    format!("---\n{yaml}---\n{body}")
}

fn read_doc<T: DeserializeOwned>(path: &Path) -> Result<(T, String)> {
    let text = std::fs::read_to_string(path)?;
    let (yaml, body) = split_frontmatter(&text).ok_or_else(|| ShapeError::Parse {
        path: path.display().to_string(),
        message: "missing or malformed YAML frontmatter".to_string(),
    })?;
    let frontmatter = serde_yaml::from_str(yaml).map_err(|err| ShapeError::Parse {
        path: path.display().to_string(),
        message: err.to_string(),
    })?;
    Ok((frontmatter, body.to_string()))
}

/// Load just the frontmatter of a document.
pub fn load<T: DeserializeOwned>(path: &Path) -> Result<T> {
    Ok(read_doc::<T>(path)?.0)
}

/// Rewrite a document's frontmatter via `update`, preserving its body.
pub fn rewrite<T, F>(path: &Path, update: F) -> Result<()>
where
    T: Serialize + DeserializeOwned,
    F: FnOnce(&mut T),
{
    let (mut frontmatter, body) = read_doc::<T>(path)?;
    update(&mut frontmatter);
    std::fs::write(path, compose(&frontmatter, &body))?;
    Ok(())
}

/// Extract the body of a `## <heading>` section (without the heading line),
/// up to the next `## ` heading or end of document.
pub fn section(body: &str, heading: &str) -> Option<String> {
    let marker = format!("## {heading}");
    let mut collecting = false;
    let mut found = false;
    let mut lines = Vec::new();
    for line in body.lines() {
        if line.starts_with("## ") {
            if collecting {
                break;
            }
            if line.trim() == marker {
                collecting = true;
                found = true;
            }
            continue;
        }
        if collecting {
            lines.push(line);
        }
    }
    found.then(|| lines.join("\n").trim().to_string())
}

fn list_md(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    if !dir.is_dir() {
        return Ok(paths);
    }
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        let is_md = path.extension().and_then(|e| e.to_str()) == Some("md");
        let is_readme = path.file_name().and_then(|n| n.to_str()) == Some("README.md");
        if is_md && !is_readme {
            paths.push(path);
        }
    }
    paths.sort();
    Ok(paths)
}

/// All pitches, sorted by path.
pub fn list_pitches(ws: &Path) -> Result<Vec<(PathBuf, Pitch)>> {
    list_md(&ws.join("pitches"))?
        .into_iter()
        .map(|path| load::<Pitch>(&path).map(|fm| (path, fm)))
        .collect()
}

/// Find a pitch by name (slugified), returning its path and frontmatter.
pub fn find_pitch(ws: &Path, name: &str) -> Result<(PathBuf, Pitch)> {
    let slug = slugify(name);
    let path = ws.join("pitches").join(format!("{slug}.md"));
    if !path.is_file() {
        return Err(ShapeError::NotFound {
            kind: "pitch",
            name: slug,
        });
    }
    let frontmatter = load::<Pitch>(&path)?;
    Ok((path, frontmatter))
}

/// All bets, sorted by path (i.e. by id).
pub fn list_bets(ws: &Path) -> Result<Vec<(PathBuf, Bet)>> {
    list_md(&ws.join("bets"))?
        .into_iter()
        .map(|path| load::<Bet>(&path).map(|fm| (path, fm)))
        .collect()
}

/// The single active bet (status `active` or `building`), if any.
pub fn active_bet(ws: &Path) -> Result<Option<(PathBuf, Bet)>> {
    Ok(list_bets(ws)?
        .into_iter()
        .find(|(_, bet)| bet.status == "active" || bet.status == "building"))
}

/// The active bet, or [`ShapeError::NoActiveBet`].
pub fn require_active_bet(ws: &Path) -> Result<(PathBuf, Bet)> {
    active_bet(ws)?.ok_or(ShapeError::NoActiveBet)
}

/// Next zero-padded bet id (`{count + 1:03}`).
pub fn next_bet_id(ws: &Path) -> Result<String> {
    Ok(format!("{:03}", list_bets(ws)?.len() + 1))
}

/// Directory holding a bet's scopes.
pub fn scopes_dir(ws: &Path, bet_id: &str) -> PathBuf {
    ws.join("scopes").join(bet_id)
}

/// All scopes of a bet, sorted by path.
pub fn list_scopes(ws: &Path, bet_id: &str) -> Result<Vec<(PathBuf, Scope)>> {
    list_md(&scopes_dir(ws, bet_id))?
        .into_iter()
        .map(|path| load::<Scope>(&path).map(|fm| (path, fm)))
        .collect()
}

/// Find a scope of a bet by name (slugified).
pub fn find_scope(ws: &Path, bet_id: &str, name: &str) -> Result<(PathBuf, Scope)> {
    let slug = slugify(name);
    let path = scopes_dir(ws, bet_id).join(format!("{slug}.md"));
    if !path.is_file() {
        return Err(ShapeError::NotFound {
            kind: "scope",
            name: slug,
        });
    }
    let frontmatter = load::<Scope>(&path)?;
    Ok((path, frontmatter))
}

/// Append-only hill-chart snapshot log for a bet.
pub fn hill_log_path(ws: &Path, bet_id: &str) -> PathBuf {
    ws.join("hill").join(format!("{bet_id}.jsonl"))
}

/// Marker file present while the workspace is in cool-down.
pub fn cooldown_path(ws: &Path) -> PathBuf {
    ws.join("cooldown")
}

/// Phase label for a hill position: `uphill` (figuring out), `downhill`
/// (executing), or `done`.
pub fn hill_phase(position: u8) -> &'static str {
    if position >= 100 {
        "done"
    } else if position >= 50 {
        "downhill"
    } else {
        "uphill"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_and_compose_round_trip() {
        let pitch = Pitch {
            name: "autopay".into(),
            title: "Autopay".into(),
            appetite: "small".into(),
            status: "shaping".into(),
            created: "2026-06-12".into(),
        };
        let doc = compose(&pitch, "\n# Autopay\n\nbody text\n");
        let (yaml, body) = split_frontmatter(&doc).expect("splits");
        assert!(yaml.contains("name: autopay"));
        assert_eq!(body, "\n# Autopay\n\nbody text\n");

        let parsed: Pitch = serde_yaml::from_str(yaml).expect("parses");
        assert_eq!(parsed.name, "autopay");
        assert_eq!(parsed.status, "shaping");
    }

    #[test]
    fn section_extracts_named_heading() {
        let body = "\n# Title\n\n## Problem\n\nthe problem\n\n## No-gos\n\nnot this\n";
        assert_eq!(section(body, "Problem").as_deref(), Some("the problem"));
        assert_eq!(section(body, "No-gos").as_deref(), Some("not this"));
        assert_eq!(section(body, "Missing"), None);
    }
}
