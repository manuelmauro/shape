//! Work with pitches — the shaped artifact (problem · appetite · solution ·
//! rabbit holes · no-gos).

use crate::cli::{Appetite, Cli, PitchArgs, PitchCommand, PitchNewArgs, PitchRefArgs};
use crate::error::{Result, ShapeError};
use crate::slug::{slugify, titleize};
use crate::workspace::{self, Pitch};
use chrono::Local;
use colored::Colorize;

/// Run the `pitch` command.
pub fn run(args: &PitchArgs, cli: &Cli) -> Result<i32> {
    match &args.command {
        PitchCommand::New(new_args) => new(new_args, cli),
        PitchCommand::List => list(cli),
        PitchCommand::Show(ref_args) => show(ref_args, cli),
        PitchCommand::Ready(ref_args) => ready(ref_args, cli),
    }
}

/// Scaffold a new pitch from the five-ingredient template.
fn new(args: &PitchNewArgs, cli: &Cli) -> Result<i32> {
    let workspace = workspace::resolve(cli)?;
    let date = Local::now().date_naive().format("%Y-%m-%d").to_string();

    let slug = slugify(&args.name);
    if slug.is_empty() {
        return Err(ShapeError::InvalidPitchName(args.name.clone()));
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

    std::fs::write(
        &path,
        render_pitch(&slug, &titleize(&slug), args.appetite, &date),
    )?;

    if !cli.quiet {
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

/// One line per pitch: `STATUS  APPETITE  NAME  TITLE` (headerless).
fn list(cli: &Cli) -> Result<i32> {
    let workspace = workspace::resolve(cli)?;
    for (_, pitch) in workspace::list_pitches(&workspace)? {
        println!(
            "{:8}{:7}{}  {}",
            pitch.status, pitch.appetite, pitch.name, pitch.title
        );
    }
    Ok(0)
}

/// Print a pitch file.
fn show(args: &PitchRefArgs, cli: &Cli) -> Result<i32> {
    let workspace = workspace::resolve(cli)?;
    let (path, _) = workspace::find_pitch(&workspace, &args.name)?;
    print!("{}", std::fs::read_to_string(&path)?);
    Ok(0)
}

/// The gate: mark a pitch shaped, once its five ingredients are filled in.
fn ready(args: &PitchRefArgs, cli: &Cli) -> Result<i32> {
    let workspace = workspace::resolve(cli)?;
    let (path, pitch) = workspace::find_pitch(&workspace, &args.name)?;

    if has_placeholder(&std::fs::read_to_string(&path)?) {
        return Err(ShapeError::PitchHasTodos { name: pitch.name });
    }

    if pitch.status == "shaped" {
        if !cli.quiet {
            println!("Pitch '{}' is already shaped.", pitch.name);
        }
        return Ok(0);
    }

    workspace::rewrite::<Pitch, _>(&path, |p| p.status = "shaped".to_string())?;

    if !cli.quiet {
        println!(
            "{} Pitch '{}' is shaped — ready to bet.",
            "✓".green().bold(),
            pitch.name
        );
        println!(
            "  Next: {}",
            format!("shape bet {}", pitch.name).as_str().bold()
        );
    }
    Ok(0)
}

/// A pitch is unfilled if any line *begins* with the `TODO:` marker the
/// template emits. Inline mentions (e.g. ``a line contains `TODO:` ``) don't
/// count — otherwise a pitch couldn't talk about the marker itself.
fn has_placeholder(text: &str) -> bool {
    text.lines()
        .any(|line| line.trim_start().starts_with("TODO:"))
}

fn render_pitch(slug: &str, title: &str, appetite: Appetite, date: &str) -> String {
    let frontmatter = Pitch {
        name: slug.to_string(),
        title: title.to_string(),
        appetite: appetite.as_str().to_string(),
        status: "shaping".to_string(),
        created: date.to_string(),
    };
    let body = format!(
        "
# {title}

<!-- A Shape Up pitch: keep it rough and bounded — boundaries, not a spec.
     Fill in the five ingredients below, then run `shape pitch ready {slug}`. -->

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
    );
    workspace::compose(&frontmatter, &body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_detection_ignores_inline_mentions() {
        assert!(has_placeholder("## Problem\n\nTODO: fill me in\n"));
        assert!(!has_placeholder(
            "Refuse if a line contains `TODO:` markers.\n"
        ));
        assert!(!has_placeholder("all filled in\n"));
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
    fn rendered_pitch_round_trips_through_frontmatter() {
        let md = render_pitch("autopay", "Autopay", Appetite::Big, "2026-06-12");
        let (yaml, _body) = workspace::split_frontmatter(&md).expect("has frontmatter");
        let pitch: Pitch = serde_yaml::from_str(yaml).expect("parses");
        assert_eq!(pitch.name, "autopay");
        assert_eq!(pitch.appetite, "big");
    }

    // Exercise the slugged write path against a temp workspace.
    #[test]
    fn render_then_write_is_loadable() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("autopay.md");
        std::fs::write(
            &path,
            render_pitch("autopay", "Autopay", Appetite::Small, "2026-06-12"),
        )
        .expect("write");
        let loaded: Pitch = workspace::load(&path).expect("load");
        assert_eq!(loaded.name, "autopay");
    }
}
