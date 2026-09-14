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

use std::iter::Peekable;
use std::path::Path;
use std::path::PathBuf;
use std::str::Chars;

use kwb_contract_tests::Repository_Root;

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

/// The widest run of spaces this repository writes on purpose, measured rather than assumed.
///
/// Five, from the aligned column in `corpus     {} current, {} held`. `KWB-57`'s lesson was that a
/// number argued from a sample of *damage* is a guess; this one is argued from the population the
/// threshold has to stay above, and `Test_The_Threshold_Should_Still_Sit_Above_Every_Legitimate_Run`
/// recomputes it from the tree so the comment cannot quietly stop being true.
const WIDEST_DELIBERATE_RUN: usize = 5;

/// How much of a sentence to quote back when the widest deliberate run has grown.
///
/// Long enough to recognise which literal moved, short enough that the failure message stays one.
const EXAMPLE_CHARS: usize = 60;

/// The fewest source files a scan of both trees could read and still be reading them.
///
/// The real count is far above this; the floor sits well below it and well above zero, which is
/// what it is for. A reader that read nothing produces a clean result that means nothing, and this
/// is the number that makes that outcome fail instead.
const MINIMUM_FILES: usize = 20;

/// The fewest literals those files could yield and still show the reader is reading text.
///
/// Same shape as `MINIMUM_FILES`, on the other measurement: a file reader that returns nothing per
/// file would clear the file floor and fail here.
const MINIMUM_LITERALS: usize = 200;

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
        if character == '\\'
        {
            Take_Escape(&mut value, &mut characters);
            continue;
        }
        value.push(character);
    }

    return value;
}

/// Take the escape the cursor is sitting on, leaving it past everything the escape covers.
///
/// # The two shapes an escape has here
///
/// A `\` followed by a newline is a *line continuation*: it eats the newline and the indentation
/// after it, and the literal goes on reading as one sentence. Any other escape is copied through
/// with what it escapes, so a `\"` does not end a literal and a `\\` does not start a continuation.
/// A trailing `\` with nothing after it is copied as the backslash it is.
fn Take_Escape(value: &mut String, characters: &mut Peekable<Chars<'_>>)
{
    if characters.peek() == Some(&'\n')
    {
        characters.next();
        Skip_Indentation(characters);
        return;
    }

    value.push('\\');
    if let Some(escaped) = characters.next()
    {
        value.push(escaped);
    }
}

/// Advance the cursor past the indentation a line continuation eats.
fn Skip_Indentation(characters: &mut Peekable<Chars<'_>>)
{
    while characters.peek().is_some_and(|next| return next.is_whitespace())
    {
        characters.next();
    }
}

/// Whether this literal's value reads with a gap in the middle of a sentence.
fn Has_A_Gap(literal: &str) -> bool
{
    let value = Value_Of(literal);
    if value.trim().chars().count() < SENTENCE
    {
        return false;
    }

    return Carries_A_Gap(value.trim());
}

/// Whether a literal already known to be long enough reads with a hole in it.
///
/// The scan is over the **trimmed** value, so a leading indent in a usage line and a trailing pad
/// are not gaps: a gap is something a reader meets *between* two words. Indentation that follows a
/// line break is not one either -- a literal carrying a newline of its own is laying something out
/// rather than writing a sentence, which is what lets the guard tests hold whole indented functions
/// in literals to feed their own detectors.
fn Carries_A_Gap(trimmed: &str) -> bool
{
    let mut run = 0_usize;
    let mut after_a_break = true;

    for character in trimmed.chars()
    {
        if character == ' '
        {
            run = run.saturating_add(1);
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
        if current.is_none()
        {
            current = (character == '"').then(String::new);
        }
        else if character == '"'
        {
            literals.extend(current.take());
        }
        else if let Some(held) = current.as_mut()
        {
            Take_Escaped_Text(held, character, &mut characters);
        }
    }

    return literals;
}

/// Push one character of a literal's body, raw, consuming the rest of an escape when this is one.
///
/// Raw, and deliberately not `Value_Of`: this reader collects literals **as written**, continuation
/// backslash and all, because `Value_Of` is the later step that decides what an escape meant. A
/// `\` here hides the character after it from the quote test, which is what keeps a `\"` from
/// ending the literal early.
fn Take_Escaped_Text(held: &mut String, character: char, characters: &mut Peekable<Chars<'_>>)
{
    held.push(character);
    if character != '\\'
    {
        return;
    }
    if let Some(escaped) = characters.next()
    {
        held.push(escaped);
    }
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
            Descend_Unless_Built(&path, into);
            continue;
        }
        if let Some(code) = Code_In(&path)
        {
            into.push((path, code));
        }
    }
}

/// Walk into a subdirectory, unless it is one of the built ones.
fn Descend_Unless_Built(path: &Path, into: &mut Vec<(PathBuf, String)>)
{
    if path.file_name().is_some_and(|name| return name == "target")
    {
        return;
    }

    Code_Of(path, into);
}

/// The comment-free code of a file a scanner may read, or `None` when the file is not one.
///
/// Comments are dropped because a doc comment's alignment table is legitimate and common here, and
/// because a comment is not a string a person is ever handed.
fn Code_In(path: &Path) -> Option<String>
{
    if path.extension().is_none_or(|extension| return extension != "rs")
    {
        return None;
    }
    if Holds_Fixtures(path)
    {
        return None;
    }

    let text = std::fs::read_to_string(path).expect("a readable source file");
    let code: String = text
        .lines()
        .filter(|line| return !line.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");

    return Some(code);
}

/// Whether this file holds deliberately damaged strings as fixtures.
///
/// A scanner cannot scan its own evidence. These two files hold damaged strings as fixtures --
/// that is what they are for, and `KWB-57` is the item that established the fixture must be the
/// real instance rather than a reconstruction. Named rather than pattern-matched, because there
/// are two of them and a pattern would quietly exempt a third file somebody added for another
/// reason.
fn Holds_Fixtures(path: &Path) -> bool
{
    return path
        .file_name()
        .is_some_and(|name| return name == "literals.rs" || name == "commands.rs");
}

/// Every comment-free line of every `.rs` file in both trees.
///
/// Both, because `crates/` is the product and `tests/` is where the guards live: a guard's own
/// assert message is read by whoever is looking at a failure, the worst moment to hand somebody a
/// sentence with a hole in it. Scanning only `crates/` missed damage that landed in this very
/// directory the day `KWB-61` was written.
fn Workspace_Code() -> Vec<(PathBuf, String)>
{
    let mut code = Vec::new();
    Code_Of(&Repository_Root().join("crates"), &mut code);
    Code_Of(&Repository_Root().join("tests"), &mut code);

    return code;
}

/// Every literal in the given files that reads with a hole in the middle of a sentence.
fn Damaged_In(code: &[(PathBuf, String)]) -> Vec<String>
{
    let mut damaged = Vec::new();
    for (path, text) in code
    {
        for literal in Literals_In(text)
        {
            if Has_A_Gap(&literal)
            {
                damaged.push(format!("{}: {literal}", path.display()));
            }
        }
    }

    return damaged;
}

#[test]
fn Test_No_Literal_Should_Read_With_A_Gap_In_The_Middle_Of_A_Sentence()
{
    let damaged = Damaged_In(&Workspace_Code());

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
    let code = Workspace_Code();

    let literals: usize = code
        .iter()
        .map(|(_, text)| return Literals_In(text).len())
        .sum();

    assert!(
        code.len() >= MINIMUM_FILES,
        "only {} source files were read, so a clean result means nothing",
        code.len()
    );
    assert!(
        literals >= MINIMUM_LITERALS,
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
    // Each of these is a real shape from this repository, and each would be a false positive that
    // got the guard turned off. Demonstrated rather than described, because an exemption nobody
    // exercised is an exemption nobody has checked.
    A_Continued_Literal_Is_Not_Damage();
    An_Aligned_Column_Is_Not_Damage();
    A_Layout_Is_Not_Damage();
}

/// A literal written correctly across lines.
///
/// In raw source this has a run of whitespace after the break, exactly like the damaged one; it
/// differs only by the backslash, which is why `Value_Of` runs before anything else.
fn A_Continued_Literal_Is_Not_Damage()
{
    let continued = "the document was accepted and could not be made durable: {cause}. \\\n                 Refusing to report a write that reached memory and not the medium";

    assert!(!Has_A_Gap(continued), "a correctly continued literal was flagged");
}

/// Printed output that aligns a column, at the widest this repository actually uses.
///
/// The second is seven spaces, one below the boundary, which is the side of it that has to hold
/// for the guard to survive contact with the code that exists.
fn An_Aligned_Column_Is_Not_Damage()
{
    assert!(!Has_A_Gap("coverage   {}"), "a printed table column was flagged");
    assert!(!Has_A_Gap("held       {}"), "a wider table column was flagged");
    assert!(
        !Has_A_Gap("corpus     {} current, {} held, and more besides to pass the length test"),
        "a table column in a literal long enough to be a sentence was flagged"
    );
}

/// Whitespace that is layout rather than a lost line break.
///
/// Three shapes, all of them here for a reason. Source held in a literal to feed another guard's
/// detector, whose indentation follows a newline the literal carries itself. A leading indent in a
/// usage line. And whitespace that is the subject rather than the separator, which the `Scope`
/// tests pass.
fn A_Layout_Is_Not_Damage()
{
    assert!(
        !Has_A_Gap("pub fn Insert(
        &mut self,
        value: usize,
) { }"),
        "a code fixture's own indentation was flagged"
    );
    assert!(
        !Has_A_Gap("       kwb supersede <concept> --into <concept> --store <dir> --because <x>"),
        "an indented usage line was flagged"
    );
    assert!(!Has_A_Gap("   "), "a literal that is itself whitespace was flagged");
    assert!(!Has_A_Gap("\t"), "a tab was flagged");
}

#[test]
fn Test_The_Threshold_Should_Still_Sit_Above_Every_Legitimate_Run()
{
    let (widest, example) = Widest_Deliberate_Run();

    // There is deliberately no `assert!(widest < GAP)` here. The measurement reports only runs
    // *below* `GAP`, so such an assertion could not fail -- a gate that passes on every tree,
    // which is the shape this repository has a name for. The scan at the top of this file is
    // what guards that side, and repeating it here would be a second authority for one question
    // rather than a second check of it.
    //
    // This is the measurement the doc comment on `GAP` states. Not an equality: a table gaining
    // a column is fine while it stays under the threshold. What must not happen unnoticed is it
    // climbing towards one.
    assert!(
        widest <= WIDEST_DELIBERATE_RUN,
        "the widest deliberate run has grown from {WIDEST_DELIBERATE_RUN} to {widest}, which is \
         still under the threshold but means the comment on GAP is now describing a tree that \
         changed: {:?}",
        Quoted_Example(&example)
    );
}

/// As much of a literal as a failure message quotes back, cut at a character boundary.
///
/// `EXAMPLE_CHARS` is the bound, and a literal that fits is quoted whole — so the ellipsis means
/// there was more of it, rather than that this was all there was.
fn Quoted_Example(example: &str) -> String
{
    let mut quoted: String = example.chars().take(EXAMPLE_CHARS).collect();
    if quoted.chars().count() < example.chars().count()
    {
        quoted.push('…');
    }

    return quoted;
}

/// The threshold's left-hand bound, recomputed from the tree on every run.
///
/// # Why a number in a comment is not enough
///
/// `GAP` is safe only while it sits above every run somebody wrote on purpose. That was
/// measured once, at five, and a measurement taken once is a claim from then on — the exact
/// shape this repository keeps finding in its own documents.
///
/// So the number is recomputed here. If a printed table ever grows a column eight wide, the test
/// above fails and says the threshold has stopped being safe, instead of the scan quietly
/// reporting somebody's deliberate alignment as damage and being switched off for it.
///
/// It reads the same literals the scan reads and reports the widest run **below** the
/// threshold, which is exactly the legitimate population — the scan at the top of this file is
/// what proves there is nothing at or above it. That is also why the test asserts nothing
/// about the threshold's upper side: a bound computed below `GAP` cannot be found to exceed it,
/// and an assertion that cannot fail is worse than none.
fn Widest_Deliberate_Run() -> (usize, String)
{
    let mut code = Vec::new();
    Code_Of(&Repository_Root().join("crates"), &mut code);

    return code
        .iter()
        .flat_map(|(_, text)| return Literals_In(text))
        .filter_map(|literal| return Widest_Legitimate_Run(&literal))
        .max_by_key(|(run, _)| return *run)
        .unwrap_or_default();
}

/// The widest run of spaces in one literal long enough to be a sentence, with that sentence, when
/// the run is **below** `GAP`; `None` when the literal carries no such run.
fn Widest_Legitimate_Run(literal: &str) -> Option<(usize, String)>
{
    let value = Value_Of(literal);
    let trimmed = value.trim();
    if trimmed.chars().count() < SENTENCE
    {
        return None;
    }

    let widest = Widest_Run_Below_The_Threshold(trimmed);
    if widest == 0
    {
        return None;
    }

    return Some((widest, trimmed.to_owned()));
}

/// The widest run of spaces in a sentence, counting only runs **below** `GAP`; zero when there is
/// none.
///
/// Runs at or above `GAP` are deliberately not counted: those are what the scan calls damage, and
/// mixing the two populations is the confusion this measurement exists to avoid.
fn Widest_Run_Below_The_Threshold(sentence: &str) -> usize
{
    let mut widest = 0_usize;
    let mut run = 0_usize;
    let mut after_a_break = true;

    for character in sentence.chars()
    {
        if character == ' '
        {
            run = run.saturating_add(1);
            if !after_a_break && run < GAP
            {
                widest = widest.max(run);
            }
            continue;
        }
        after_a_break = character == '\n';
        run = 0;
    }

    return widest;
}
