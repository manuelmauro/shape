//! Work with pitches — the shaped artifact.
//!
//! `pitch new` is implemented: it scaffolds `pitches/<slug>.md` from the
//! five-ingredient template. `list`, `show`, and `ready` are still stubbed.

use crate::cli::{Appetite, Cli, PitchArgs, PitchCommand, PitchNewArgs};
use crate::config;
use crate::error::{Result, ShapeError};
use chrono::Local;
use colored::Colorize;
use serde::Serialize;
use std::path::{Path, PathBuf};

/// Run the `pitch` command.
pub fn run(args: &PitchArgs, cli: &Cli) -> Result<i32> {
    match &args.command {
        PitchCommand::New(new_args) => new(new_args, cli),
        PitchCommand::List => super::unimplemented_bare("pitch list", cli),
        PitchCommand::Show(ref_args) => super::unimplemented("pitch show", ref_args, cli),
        PitchCommand::Ready(ref_args) => super::unimplemented("pitch ready", ref_args, cli),
    }
}

/// Scaffold a new pitch from the five-ingredient template.
fn new(args: &PitchNewArgs, cli: &Cli) -> Result<i32> {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let workspace = config::effective_dir(cli.dir.as_deref(), &cwd)?;
    let date = Local::now().date_naive().format("%Y-%m-%d").to_string();

    let path = create_pitch(&workspace, &args.name, args.appetite, &date)?;

    if !cli.quiet {
        let slug = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        println!(
            "{} Created pitch '{}' at {}",
            "✓".green().bold(),
            slug,
            path.display()
        );
        println!(
            "  Fill in the five ingredients, then run {}",
            format!("shape pitch ready {slug}").as_str().bold()
        );
    }
    Ok(0)
}

/// Write `pitches/<slug>.md` under `workspace`, returning the path. Errors if
/// the name has no usable slug or a pitch with that slug already exists.
fn create_pitch(
    workspace: &Path,
    raw_name: &str,
    appetite: Appetite,
    date: &str,
) -> Result<PathBuf> {
    let slug = slugify(raw_name);
    if slug.is_empty() {
        return Err(ShapeError::InvalidPitchName(raw_name.to_string()));
    }

    let pitches_dir = workspace.join("pitches");
    std::fs::create_dir_all(&pitches_dir)?;

    let path = pitches_dir.join(format!("{slug}.md"));
    if path.exists() {
        return Err(ShapeError::PitchExists {
            name: slug,
            path: path.display().to_string(),
        });
    }

    let title = titleize(&slug);
    std::fs::write(&path, render_pitch(&slug, &title, appetite, date))?;
    Ok(path)
}

#[derive(Serialize)]
struct PitchFrontmatter<'a> {
    name: &'a str,
    title: &'a str,
    appetite: &'a str,
    status: &'a str,
    created: &'a str,
}

fn render_pitch(name: &str, title: &str, appetite: Appetite, date: &str) -> String {
    let frontmatter = PitchFrontmatter {
        name,
        title,
        appetite: appetite.as_str(),
        status: "shaping",
        created: date,
    };
    let yaml = serde_yaml::to_string(&frontmatter)
        .expect("pitch frontmatter serialization is infallible for static fields");

    format!(
        "---
{yaml}---

# {title}

<!-- A Shape Up pitch: keep it rough and bounded — boundaries, not a spec.
     Fill in the five ingredients below, then run `shape pitch ready {name}`. -->

## Problem

TODO: one concrete story showing why the status quo fails today (the baseline)
— what breaks, for whom, and when it actually bit.

## Appetite

**{label}** — {summary}.

TODO: why this appetite? What does it rule in, and what does it rule out?

## Solution

TODO: the rough shape of the solution. Breadboard the flow — places,
affordances, and connections — or sketch it at low fidelity. Leave room.

## Rabbit holes

TODO: risks and unknowns to patch or fence now, so the build doesn't grind on
them later.

-

## No-gos

TODO: what's explicitly out of bounds to fit the appetite or keep it tractable.

-
",
        label = appetite.label(),
        summary = appetite.summary(),
    )
}

/// Reduce arbitrary text to a kebab-case slug: lowercase ASCII alphanumerics,
/// runs of anything else collapse to a single hyphen, with no leading or
/// trailing hyphen.
fn slugify(input: &str) -> String {
    let mut slug = String::new();
    let mut pending_dash = false;
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            if pending_dash && !slug.is_empty() {
                slug.push('-');
            }
            slug.push(ch.to_ascii_lowercase());
            pending_dash = false;
        } else {
            pending_dash = true;
        }
    }
    slug
}

/// Turn a slug into a human title: `auto-pay` → `Auto Pay`.
fn titleize(slug: &str) -> String {
    slug.split('-')
        .filter(|word| !word.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_normalizes() {
        assert_eq!(slugify("Auto Pay!"), "auto-pay");
        assert_eq!(slugify("autopay"), "autopay");
        assert_eq!(slugify("  multi   word  "), "multi-word");
        assert_eq!(slugify("Trailing--dash--"), "trailing-dash");
        assert_eq!(slugify("!!!"), "");
    }

    #[test]
    fn titleize_capitalizes_words() {
        assert_eq!(titleize("auto-pay"), "Auto Pay");
        assert_eq!(titleize("autopay"), "Autopay");
    }

    #[test]
    fn render_pitch_has_frontmatter_and_five_ingredients() {
        let md = render_pitch("autopay", "Autopay", Appetite::Small, "2026-06-12");
        assert!(md.contains("name: autopay"));
        assert!(md.contains("appetite: small"));
        assert!(md.contains("status: shaping"));
        assert!(md.contains("Small Batch"));
        for section in [
            "## Problem",
            "## Appetite",
            "## Solution",
            "## Rabbit holes",
            "## No-gos",
        ] {
            assert!(md.contains(section), "missing {section}");
        }
    }

    #[test]
    fn create_pitch_writes_file_and_rejects_duplicates() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path =
            create_pitch(tmp.path(), "Auto Pay", Appetite::Big, "2026-06-12").expect("create");

        assert_eq!(path, tmp.path().join("pitches/auto-pay.md"));
        assert!(path.is_file());

        let err = create_pitch(tmp.path(), "auto-pay", Appetite::Big, "2026-06-12")
            .expect_err("duplicate should error");
        assert!(matches!(err, ShapeError::PitchExists { .. }));
    }

    #[test]
    fn create_pitch_rejects_empty_slug() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let err = create_pitch(tmp.path(), "!!!", Appetite::Small, "2026-06-12")
            .expect_err("invalid name should error");
        assert!(matches!(err, ShapeError::InvalidPitchName(_)));
    }
}
