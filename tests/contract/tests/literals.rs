//! What a string literal says once Rust has finished reading it.
//!
//! # The defect this scans for
//!
//! A Rust literal continued with a trailing `\` strips the newline **and the indentation that
//! follows it**. Several other languages continue their strings with the same character, so an
//! edit written in one of them consumes the backslash and the newline before the file is
//! written, and keeps the indentation. What lands is a single-line literal carrying a run of
//! spaces in the middle of a sentence.
//!
//! It compiles. Clippy passes. Every test passes. It is visible only when the string is
//! printed — and on 2026-09-12 five of them landed across four crates in one session, one
//! reached actual command-line output, and one survived four readings of the source, because a
//! reader's eye reconstructs the sentence it expected to find.
//!
//! # Why this is a scan and not a test per type
//!
//! `KWB-51` guarded `StoreError` by rendering every variant. That covers one type, and the
//! defect is not a property of a type: `ExtractionRefused`, `ReplayError`, `IdentityError` and
//! `StorageError` all reach a person through `Display`, and every assert message in the
//! workspace reaches whoever is reading a failure.
//!
//! # What `KWB-44` requires of it
//!
//! A guard here is held to *showing* it detects what it claims to. A scanner that silently
//! reads nothing passes on every repository, and this file's last three tests exist to make
//! that impossible to mistake for a clean result.

use std::path::Path;
use std::path::PathBuf;

/// The repository root, from this test's own manifest.
fn Repository_Root() -> PathBuf
{
    return PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("tests/contract sits two levels below the root")
        .to_path_buf();
}

/// How many spaces in a row are a gap rather than an alignment.
///
/// # The two numbers that bound it
///
/// A threshold here is safe when it sits **above every legitimate run** and **at or below the
/// shallowest damage**. Both are properties of the tree and both are measured:
///
/// | | |
/// |---|---|
/// | largest run inside a literal of 30+ characters | **5** — `corpus     {} current, {} held` |
/// | shallowest continuation indent in `crates/` | **9** — 43 sites, at 9, 13 and 17 |
///
/// Eight sits between them. `Test_The_Threshold_Should_Still_Sit_Above_Every_Legitimate_Run`
/// keeps the left-hand number honest, so this table cannot quietly stop being true.
///
/// # The blind spot, stated because it is not zero
///
/// Damage is the **source indentation of whatever line it happened to**, so its magnitude is a
/// property of the code and not of the defect — it has no floor in principle. A continuation
/// site indented six, seven or eight would produce damage this threshold misses. **No such site
/// exists**: the shallowest is nine. If one is ever written, this guard goes quiet about it and
/// says nothing, which is the failure mode worth knowing about in advance.
///
/// # Why the justification was rewritten
///
/// It used to read *"the five real instances carried ten, ten, ten, ten and eighteen"* — the
/// number argued from a sample of damage. `KWB-57` carried that same reasoning to markdown
/// command lines, where the real instance carried **seven**, the guard reported clean on it, and
/// its own detector test passed because the fixture was a reconstruction that happened to carry
/// ten. The number here was right and its reason was the reason that failed, so the reason is
/// what changed.
const GAP: usize = 8;

/// How long a literal has to be before a run of spaces in it is a sentence and not a layout.
///
/// A short literal padded with spaces is a column, a separator or a piece of whitespace being
/// tested for its own sake. A long one with a hole in the middle is a sentence that lost a line
/// break.
const SENTENCE: usize = 30;

/// The literal's value, as Rust reads it: a `\` at end of line eats the newline and the
/// indentation after it.
///
/// Doing this *first* is what separates a correctly continued literal from a damaged one. Both
/// look the same in raw source — a run of whitespace after a line break — and they differ only
/// in whether the backslash is there. A scanner that skipped this step would flag every
/// well-written multi-line message in the workspace and be turned off within a day.
fn Value_Of(source: &str) -> String
{
    let mut value = String::with_capacity(source.len());
    let mut characters = source.chars().peekable();

    while let Some(character) = characters.next()
    {
        if character != '\\'
        {
            value.push(character);
            continue;
        }

        match characters.peek()
        {
            Some('\n') =>
            {
                characters.next();
                while characters.peek().is_some_and(|next| return next.is_whitespace())
                {
                    characters.next();
                }
            }
            // Any other escape is copied through with what it escapes, so a `\"` does not end a
            // literal and a `\\` does not start a continuation.
            Some(_) =>
            {
                value.push(character);
                if let Some(escaped) = characters.next()
                {
                    value.push(escaped);
                }
            }
            None => value.push(character),
        }
    }

    return value;
}

/// Whether this literal's value reads with a gap in the middle of a sentence.
fn Has_A_Gap(literal: &str) -> bool
{
    let value = Value_Of(literal);
    if value.trim().chars().count() < SENTENCE
    {
        return false;
    }

    // Trimmed, so a leading indent in a usage line and a trailing pad are not gaps: a gap is
    // something a reader meets *between* two words.
    let trimmed = value.trim();
    let mut run = 0_usize;
    let mut after_a_break = true;

    for character in trimmed.chars()
    {
        if character == ' '
        {
            run = run.saturating_add(1);
            // Indentation follows a line break, and a literal carrying a newline of its own is
            // laying something out rather than writing a sentence. The guard tests hold whole
            // functions in literals to feed their own detectors, and those are indented.
            if run >= GAP && !after_a_break
            {
                return true;
            }
            continue;
        }

        after_a_break = character == '\n';
        run = 0;
    }

    return false;
}

/// Every string literal in a file, as written, including its line continuations.
fn Literals_In(source: &str) -> Vec<String>
{
    let mut literals = Vec::new();
    let mut characters = source.chars().peekable();
    let mut current: Option<String> = None;

    while let Some(character) = characters.next()
    {
        match current
        {
            None =>
            {
                if character == '"'
                {
                    current = Some(String::new());
                }
            }
            Some(ref mut held) =>
            {
                if character == '\\'
                {
                    held.push(character);
                    if let Some(escaped) = characters.next()
                    {
                        held.push(escaped);
                    }
                    continue;
                }
                if character == '"'
                {
                    literals.push(held.clone());
                    current = None;
                    continue;
                }
                held.push(character);
            }
        }
    }

    return literals;
}

/// Every line of every `.rs` file under a directory that is not a comment.
///
/// Comments are dropped because a doc comment's alignment table is legitimate and common here,
/// and because a comment is not a string a person is ever handed.
fn Code_Of(directory: &Path, into: &mut Vec<(PathBuf, String)>)
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
            if path.file_name().is_some_and(|name| return name == "target")
            {
                continue;
            }
            Code_Of(&path, into);
            continue;
        }
        if path.extension().is_none_or(|extension| return extension != "rs")
        {
            continue;
        }

        let text = std::fs::read_to_string(&path).expect("a readable source file");
        let code: String = text
            .lines()
            .filter(|line| return !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        into.push((path, code));
    }
}

#[test]
fn Test_No_Literal_Should_Read_With_A_Gap_In_The_Middle_Of_A_Sentence()
{
    let mut code = Vec::new();
    Code_Of(&Repository_Root().join("crates"), &mut code);

    let mut damaged: Vec<String> = Vec::new();
    for (path, text) in &code
    {
        for literal in Literals_In(text)
        {
            if Has_A_Gap(&literal)
            {
                damaged.push(format!("{}: {literal}", path.display()));
            }
        }
    }

    assert!(
        damaged.is_empty(),
        "these literals carry a run of spaces where a line break was, so whatever prints them \
         prints a sentence with a hole in it: {damaged:#?}"
    );
}

// ---- KWB-44: the scanner is shown to read, and shown to detect ----

#[test]
fn Test_The_Scan_Should_Actually_Have_Read_The_Workspace()
{
    let mut code = Vec::new();
    Code_Of(&Repository_Root().join("crates"), &mut code);

    let literals: usize = code
        .iter()
        .map(|(_, text)| return Literals_In(text).len())
        .sum();

    assert!(
        code.len() >= 20,
        "only {} source files were read, so a clean result means nothing",
        code.len()
    );
    assert!(
        literals >= 200,
        "only {literals} literals were found in {} files, so the reader is not reading",
        code.len()
    );
}

#[test]
fn Test_The_Detector_Should_Find_The_Damage_It_Was_Written_For()
{
    // Reconstructed from `StoreError::NotStored`, which is what a person is told when a
    // document was accepted and could not be made durable. This is the text that was actually
    // in the repository, not an invented example.
    let damaged = "the document was accepted and could not be made durable: {cause}. Refusing \
                   to                  report a write that reached memory and not the medium";

    assert!(
        Has_A_Gap(damaged),
        "the detector does not find the instance it was written for, so a clean scan of the \
         workspace says nothing about the workspace"
    );
}

#[test]
fn Test_The_Detector_Should_Not_Find_What_Is_Not_Damage()
{
    // Each of these is a real shape from this repository, and each would be a false positive
    // that got the guard turned off. Demonstrated rather than described, because a exemption
    // nobody exercised is an exemption nobody has checked.

    // A literal written correctly across lines. In raw source this has a run of whitespace
    // after the break, exactly like the damaged one; it differs only by the backslash, which is
    // why `Value_Of` runs before anything else.
    let continued = "the document was accepted and could not be made durable: {cause}. \\\n                 Refusing to report a write that reached memory and not the medium";
    assert!(!Has_A_Gap(continued), "a correctly continued literal was flagged");

    // An aligned column in printed output, at the widest this repository actually uses. The
    // second is seven spaces, one below the boundary, which is the side of it that has to hold
    // for the guard to survive contact with the code that exists.
    assert!(!Has_A_Gap("coverage   {}"), "a printed table column was flagged");
    assert!(!Has_A_Gap("held       {}"), "a wider table column was flagged");
    assert!(
        !Has_A_Gap("corpus     {} current, {} held, and more besides to pass the length test"),
        "a table column in a literal long enough to be a sentence was flagged"
    );

    // Source held in a literal to feed another guard's detector. Its indentation follows a
    // newline the literal carries itself, which is layout rather than a lost line break.
    assert!(
        !Has_A_Gap("pub fn Insert(
        &mut self,
        value: usize,
) { }"),
        "a code fixture's own indentation was flagged"
    );

    // A leading indent in a usage line, which is a layout and not a gap.
    assert!(
        !Has_A_Gap("       kwb supersede <concept> --into <concept> --store <dir> --because <x>"),
        "an indented usage line was flagged"
    );

    // Whitespace that is the subject rather than the separator -- `Scope` tests pass these.
    assert!(!Has_A_Gap("   "), "a literal that is itself whitespace was flagged");
    assert!(!Has_A_Gap("\t"), "a tab was flagged");
}

/// The threshold's left-hand bound, recomputed from the tree on every run.
///
/// # Why a number in a comment is not enough
///
/// `GAP` is safe only while it sits above every run somebody wrote on purpose. That was
/// measured once, at five, and a measurement taken once is a claim from then on — the exact
/// shape this repository keeps finding in its own documents.
///
/// So the number is recomputed here. If a printed table ever grows a column eight wide, this
/// fails and says the threshold has stopped being safe, instead of the scan above quietly
/// reporting somebody's deliberate alignment as damage and being switched off for it.
///
/// It reads the same literals the scan reads and reports the widest run **below** the
/// threshold, which is exactly the legitimate population — the scan at the top of this file is
/// what proves there is nothing at or above it. That is also why this test asserts nothing
/// about the threshold's upper side: a bound computed below `GAP` cannot be found to exceed it,
/// and an assertion that cannot fail is worse than none.
#[test]
fn Test_The_Threshold_Should_Still_Sit_Above_Every_Legitimate_Run()
{
    let mut code = Vec::new();
    Code_Of(&Repository_Root().join("crates"), &mut code);

    let mut widest = 0_usize;
    let mut example = String::new();
    for (_, text) in &code
    {
        for literal in Literals_In(text)
        {
            let value = Value_Of(&literal);
            let trimmed = value.trim();
            if trimmed.chars().count() < SENTENCE
            {
                continue;
            }

            let mut run = 0_usize;
            let mut after_a_break = true;
            for character in trimmed.chars()
            {
                if character == ' '
                {
                    run = run.saturating_add(1);
                    if !after_a_break && run > widest && run < GAP
                    {
                        widest = run;
                        example = trimmed.chars().take(60).collect();
                    }
                    continue;
                }
                after_a_break = character == '\n';
                run = 0;
            }
        }
    }

    // There is deliberately no `assert!(widest < GAP)` here. The loop above records only runs
    // *below* `GAP`, so such an assertion could not fail -- a gate that passes on every tree,
    // which is the shape this repository has a name for. The scan at the top of this file is
    // what guards that side, and repeating it here would be a second authority for one question
    // rather than a second check of it.
    //
    // This is the measurement the doc comment on `GAP` states. Not an equality: a table gaining
    // a column is fine while it stays under the threshold. What must not happen unnoticed is it
    // climbing towards one.
    assert!(
        widest <= 5,
        "the widest deliberate run has grown from 5 to {widest}, which is still under the \
         threshold but means the comment on GAP is now describing a tree that changed: \
         {example:?}"
    );
}
