//! The command lines this repository tells a reader to type.
//!
//! # Why these and not markdown generally
//!
//! `KWB-52` built a scanner for a string literal whose line continuation had been eaten by a
//! scripted edit — the backslash and newline consumed, the indentation kept, leaving a run of
//! spaces in the middle of a sentence. It reads `.rs` files under `crates/`.
//!
//! `KWB-56` found the fifth instance outside that reach: both `kwb admit` lines in `README.md`
//! carried seven spaces before `--says`, in the one command a new reader is asked to run. The
//! damage is a property of an edit and not of a language, so anything a scripted edit writes
//! can lose a continuation — and a command a reader copies is the worst place for it to land.
//!
//! Markdown is not Rust and the Rust rule does not transfer. Indentation is meaningful, tables
//! are aligned on purpose, and a fenced block holds text whose layout *is* its content — this
//! repository's own console output aligns `coverage   yielded` into columns. A scan that
//! flagged those would be turned off, which is what `KWB-52`'s exemptions were written against.
//!
//! So this scans one thing: a line that begins with `$ `. That is a command, unambiguously,
//! wherever it appears — no fence language to enumerate, no alignment to exempt, and exactly
//! the line a reader selects and pastes.

use std::path::Path;
use std::path::PathBuf;

use kwb_contract_tests::Repository_Root;

/// How many spaces in a row are damage rather than layout.
///
/// # Measured for *this* subject, after inheriting the wrong number
///
/// This said eight, taken from `KWB-52` where it was measured against Rust literals: the real
/// instances there carried ten or more and deliberate column alignment was seven at the widest.
///
/// **That threshold would have missed the instance this file exists for.** The damage `KWB-56`
/// found in `README.md` was *seven* spaces, and the detector test below passed anyway because it
/// used a reconstruction that happened to carry ten -- so the demonstration was green while the
/// real thing slipped through. Found by reintroducing the actual damage and watching the scan
/// report clean.
///
/// Measured properly: across every command line in this repository the longest run of spaces is
/// **zero**. Nobody types a run inside a command, so there is no alignment to protect and the
/// boundary belongs at the floor. Two, with the whole tree already clean of it.
///
/// The general lesson is `KWB-52`'s own: a threshold carried from one subject to another is a
/// guess wearing a measurement's clothes.
const GAP: usize = 2;

/// The fewest command lines a scan could find and still be reading something.
///
/// Seven command lines across three files when this was written, so the floor sits well below the
/// real count and well above zero. That is the whole point of it: a scanner that reads nothing
/// passes on every repository, and this is the number that makes that outcome fail.
const MINIMUM_COMMANDS: usize = 5;

/// The fewest files those command lines can be spread across and still show the walk is walking.
///
/// Three files when this was written. Separate from the command floor because the two failures it
/// separates are different: finding too few commands is a reader that stopped early, and finding
/// them all in one file is a walk that never descended.
const MINIMUM_FILES: usize = 2;

/// Whether a command line has a hole in it.
///
/// Trimmed, so an indented command in a nested block is not a finding: what matters is a run a
/// reader meets *between* two arguments.
fn Has_A_Gap(command: &str) -> bool
{
    let mut run = 0_usize;

    for character in command.trim().chars()
    {
        if character == ' '
        {
            run = run.saturating_add(1);
            if run >= GAP
            {
                return true;
            }
            continue;
        }
        run = 0;
    }

    return false;
}

/// Every markdown file under the repository, minus anything built.
fn Prose(directory: &Path, into: &mut Vec<(PathBuf, String)>)
{
    let Ok(entries) = std::fs::read_dir(directory)
    else
    {
        return;
    };

    for entry in entries
    {
        let path = entry.expect("a readable directory entry").path();
        if path.is_dir()
        {
            let skip = path
                .file_name()
                .is_some_and(|name| return name == "target" || name == ".git");
            if !skip
            {
                Prose(&path, into);
            }
            continue;
        }
        if path.extension().is_some_and(|extension| return extension == "md")
        {
            let text = std::fs::read_to_string(&path).expect("a readable markdown file");
            into.push((path, text));
        }
    }
}

/// Every line this repository presents as a command to type.
fn Commands() -> Vec<(PathBuf, String)>
{
    let mut prose = Vec::new();
    Prose(&Repository_Root(), &mut prose);

    let mut commands = Vec::new();
    for (path, text) in prose
    {
        for line in text.lines()
        {
            if line.trim_start().starts_with("$ ")
            {
                commands.push((path.clone(), line.to_owned()));
            }
        }
    }

    return commands;
}

#[test]
fn Test_No_Command_A_Reader_Is_Told_To_Type_Should_Have_A_Hole_In_It()
{
    let damaged: Vec<String> = Commands()
        .into_iter()
        .filter(|(_, command)| return Has_A_Gap(command))
        .map(|(path, command)| return format!("{}: {command}", path.display()))
        .collect();

    assert!(
        damaged.is_empty(),
        "these are commands this repository tells a reader to type, and they carry a run of \
         spaces where a line break was: {damaged:#?}"
    );
}

#[test]
fn Test_The_Scan_Should_Actually_Have_Found_The_Commands()
{
    // A scanner that reads nothing passes on every repository. This one has a small subject, so
    // the floor is low and the point is that it is not zero.
    let commands = Commands();
    let files: std::collections::BTreeSet<&PathBuf> =
        commands.iter().map(|(path, _)| return path).collect();

    assert!(
        commands.len() >= MINIMUM_COMMANDS,
        "only {} command lines were found, so a clean result means nothing",
        commands.len()
    );
    assert!(
        files.len() >= MINIMUM_FILES,
        "commands were found in only {} file, so the walk is not walking",
        files.len()
    );
}

#[test]
fn Test_The_Detector_Should_Find_The_Damage_It_Was_Written_For()
{
    // The instance exactly as `KWB-56` found it in `README.md` -- seven spaces, not a round
    // number chosen to make a test pass. An earlier version of this test used a reconstruction
    // carrying ten, which passed against a threshold of eight that would have let the real one
    // through. A demonstration built from an invented instance demonstrates nothing.
    //
    // Assembled with `concat!` so the run is a literal of its own and cannot be reflowed by an
    // editor, a formatter, or the kind of scripted edit that caused the damage in the first
    // place.
    let damaged = concat!(
        "$ kwb admit callen.txt --store ./corpus --scope \"physical theory\"",
        "       ",
        "--says entropy \"It is non-decreasing in an isolated system.\""
    );

    assert!(
        Has_A_Gap(damaged),
        "the detector does not find the instance it was written for, so a clean scan says \
         nothing about the repository"
    );
}

#[test]
fn Test_The_Detector_Should_Not_Find_What_Is_Not_Damage()
{
    // Real shapes from this repository. Demonstrated rather than described, because an
    // exemption nobody exercised is an exemption nobody has checked.
    assert!(
        !Has_A_Gap("$ kwb admit callen.txt --store ./corpus --scope \"physical theory\" --says entropy \"x\""),
        "the repaired command was flagged"
    );
    assert!(
        !Has_A_Gap("$ nomos work add --item KWB-17 --amends D-004 ..."),
        "a command from an observation was flagged"
    );
    assert!(
        !Has_A_Gap("    $ kwb-mcp ./corpus neighbours entropy"),
        "an indented command was flagged, so indentation is being read as a hole"
    );
}
