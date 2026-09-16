//! What `kwb help` prints, and where each line of it begins.
//!
//! # Why this is here and not in `src/usage.rs`
//!
//! `kwb-cli` is a binary with no library target, so nothing outside it can call `Print_Usage` —
//! it is `pub(crate)`, and there is no crate to link against. What can drive it is running the
//! binary, because `kwb help` is `main` calling `Print_Usage` and nothing else: the rendering is
//! the whole of what that path does. So it is driven that way, from a file whose stem is the unit
//! the rule reads, which is the only arrangement in this crate that puts the two together.
//!
//! # What this asserts that `tests/help.rs` does not
//!
//! `tests/help.rs` checks *which* verbs the help offers. It does not check the layout — and the
//! layout is the one thing `Print_Usage` knows that `VERBS` does not, which its own documentation
//! says: the loop is what puts the label on the first line and aligns the rest under it. Nothing
//! tested that, so a renderer that dropped the label, or repeated it on every line, or padded the
//! verbs to different columns, would compile and pass every other test in the crate.

use std::process::Command;

/// The binary this test was built alongside.
const KWB: &str = env!("CARGO_BIN_EXE_kwb");

/// The label a reader scans the help for: the whole of what stands to the left of the verb on the
/// first usage line, separator included.
const LABEL: &str = "usage: ";

/// Where the verb begins on every usage line: under the label, not after it.
///
/// Spelled as the label's own length rather than as `7` — the label is what decides the column, and
/// a literal here would keep agreeing with itself after the label stopped agreeing with the
/// renderer.
const VERB_COLUMN: usize = LABEL.len();

/// What every usage line names before it says what the verb takes.
const TOOL: &str = "kwb ";

/// How many usage lines the help must offer before the layout below is worth reading.
///
/// A floor on how much is being checked, not a claim about what the table declares: a renderer
/// reduced to a single line would satisfy every assertion here and mean nothing by it.
const FEWEST_USAGE_LINES: usize = 4;

/// What `kwb help` prints, from the binary.
fn Help() -> String
{
    let printed = Command::new(KWB)
        .arg("help")
        .output()
        .expect("the binary this test was built alongside should run");

    // The usage goes to standard error, which is where a usage belongs; a reader sees both, so
    // both are read rather than one of them being assumed.
    return format!(
        "{}{}",
        String::from_utf8_lossy(&printed.stdout),
        String::from_utf8_lossy(&printed.stderr)
    );
}

/// Every line of the help that offers a verb, in the order they were printed.
fn Usage_Lines(help: &str) -> Vec<&str>
{
    return help
        .lines()
        .filter(|line| return Is_Usage_Line(line))
        .collect();
}

/// Whether a line offers a verb: the tool at the verb's column, behind the label or the blanks
/// that stand in for it.
///
/// What is to the left of the column is checked and not just skipped, and that is load-bearing
/// rather than tidy. The prose under the table is indented, and one line of it names `kwb-extract`
/// — so a filter that looked for `kwb ` anywhere on the line would read that sentence as a usage
/// and report a layout the help does not have.
fn Is_Usage_Line(line: &str) -> bool
{
    let (Some(label), Some(verb)) = (line.get(..VERB_COLUMN), line.get(VERB_COLUMN..))
    else
    {
        return false;
    };

    // Either the label speaks for this line, or the blanks the renderer puts in its place do.
    return verb.starts_with(TOOL) && (label == LABEL || label.trim().is_empty());
}

/// The floor on how much layout the test below is actually checking.
///
/// Separated from the two claims about the layout because it is not one: a renderer reduced to a
/// single line would satisfy both of them and mean nothing by it, so this is the guard on whether
/// they are worth reading at all.
fn Assert_Enough_Lines(usage: &[&str])
{
    assert!(
        usage.len() >= FEWEST_USAGE_LINES,
        "only {} usage line(s) were found, so this guard is checking almost no layout: {usage:?}",
        usage.len()
    );
}

/// The label stands on the first usage line and on no other.
///
/// Both halves are one claim. A renderer that dropped the label leaves the rest with nothing to
/// align under, and one that repeated it on every line has not aligned anything — the second is
/// the failure the alignment exists to avoid, and it is only visible once the first is known.
fn Assert_Only_The_First_Line_Carries_The_Label(usage: &[&str])
{
    let (first, rest) = usage.split_first().expect("a usage line to label");
    assert!(
        first.starts_with(LABEL),
        "the help does not label its first usage line, so there is nothing for the rest to align \
         under: {first}"
    );

    for line in rest
    {
        assert!(
            !line.starts_with(LABEL),
            "the label was repeated on a line after the first, which is the one thing the \
             alignment exists to avoid: {line}"
        );
    }
}

/// Every verb begins at the label's own column, so two usage lines compare line for line.
fn Assert_Every_Verb_Starts_At_The_Label_Column(usage: &[&str])
{
    for line in usage
    {
        let at = line.find(TOOL).expect("a usage line names the tool it is a usage for");

        assert_eq!(
            at, VERB_COLUMN,
            "a verb does not begin where the label ends, so the usage lines do not line up and a \
             reader comparing two of them cannot: {line}"
        );
    }
}

#[test]
fn Test_Print_Usage_Should_Label_The_First_Usage_Line_And_Align_The_Rest_Under_It()
{
    let help = Help();
    let usage = Usage_Lines(&help);

    Assert_Enough_Lines(&usage);
    Assert_Only_The_First_Line_Carries_The_Label(&usage);
    Assert_Every_Verb_Starts_At_The_Label_Column(&usage);
}
