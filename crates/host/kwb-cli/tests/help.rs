//! What the tool tells a person it can do, read from the tool.
//!
//! # Why this runs the binary instead of reading the source
//!
//! The defect this exists for is that somebody running `kwb help` is told something false.
//! A check on `Print_Usage`'s source would pass on a help string that is assembled and never
//! emitted, or emitted on a path nobody reaches — it would be checking that the right words
//! exist somewhere, which is not the claim.
//!
//! It is also the third surface. `README.md` and the dispatch are checked against each other in
//! both directions by `tests/contract`, and neither of those checks reads the help. `KWB-67`
//! measured what that missed: `kwb help` named `history` **zero** times while the dispatch
//! carried it and the README documented it twice — and the README routes a reader to the help
//! for flags, saying *a copy of a help text is a copy that drifts*.
//!
//! # What changed under `KWB-73`, and why these tests survive it
//!
//! The drift is now impossible by construction rather than detected after the fact: `VERBS`
//! declares each verb with its usage line, `main` dispatches by looking it up, and
//! `Print_Usage` renders it. There is one list, so a verb cannot be dispatched and unlisted.
//!
//! These tests are not redundant, because a projection is only a projection while something
//! **emits** it. `Print_Usage` could stop rendering the table — an early return, a `#[cfg]`, or
//! somebody putting hand-written lines back — and none of that would fail to compile. So the
//! binary is still run and what it printed is still read.

use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

/// The binary this test was built alongside.
const KWB: &str = env!("CARGO_BIN_EXE_kwb");

/// What `kwb help` prints, from the binary.
fn Help() -> String
{
    let printed = Command::new(KWB)
        .arg("help")
        .output()
        .expect("the binary this test was built alongside should run");

    // The usage goes to standard error, which is where a usage belongs; a reader sees both, so
    // this checks both rather than assuming which stream carried it.
    return format!(
        "{}{}",
        String::from_utf8_lossy(&printed.stdout),
        String::from_utf8_lossy(&printed.stderr)
    );
}

/// Every verb the table declares.
///
/// # What this guard covers now, which is not what it covered before
///
/// Until `KWB-73` the dispatch and the help each listed the verbs, and both directions of this
/// file compared the two lists. `KWB-73` made them one: `VERBS` declares each verb with its
/// usage line, `main` dispatches by looking it up, and `Print_Usage` renders it. **The name
/// direction is now structural** — a verb cannot be dispatched and unlisted, because there is
/// one list.
///
/// What is still worth checking is that the projection is *emitted*. A table nobody renders is
/// not a projection, and `Print_Usage` could stop rendering it — by an early return, by a
/// `#[cfg]`, or by somebody replacing the loop with hand-written lines again — without any of
/// that failing to compile. So these tests still run the binary and read what it printed.
///
/// The parse is of `VERBS` rather than of match arms, because that is where the canonical
/// declaration moved. Reading the source at all is a concession: `kwb-cli` is a binary with no
/// library target, so a test cannot name the value directly the way `tests/contract` names
/// `kwb_mcp::TOOLS`.
fn Dispatched() -> Vec<String>
{
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dispatch = std::fs::read_to_string(Path::new(&root).join("src/main.rs"))
        .expect("the composition root should be readable");

    let Some(table) = dispatch.split_once("const VERBS:").map(|(_, rest)| return rest)
    else
    {
        return Vec::new();
    };
    let table = table
        .split_once("\n];")
        .map_or(table, |(inside, _)| return inside);

    let mut verbs: Vec<String> = Vec::new();
    for fragment in table.split("name: \"").skip(1)
    {
        let Some(name) = fragment.split('"').next()
        else
        {
            continue;
        };
        if !verbs.iter().any(|held| return held == name)
        {
            verbs.push(name.to_owned());
        }
    }

    return verbs;
}

/// Every verb the help offers on a usage line, as a reader scanning for one would find it.
///
/// # Why a usage line and not the whole text
///
/// The first version of the test below asked whether the help *contained* the verb anywhere. It
/// passed against a help with the usage line deleted, because the prose still said *asking for
/// more history than the log holds* — a verb whose name is an ordinary English word is satisfied
/// by accident. `KWB-55` is the item where a check that looked for an identifier anywhere in a
/// document let a stale section through; the same weakness, rebuilt by the session that removed
/// it, and caught by mutating the real file rather than by review.
fn Offered(help: &str) -> Vec<String>
{
    let mut offered: Vec<String> = Vec::new();
    for line in help.lines()
    {
        let trimmed = line.trim().trim_start_matches("usage:").trim();
        let Some(rest) = trimmed.strip_prefix("kwb ")
        else
        {
            continue;
        };
        if let Some(verb) = rest.split_whitespace().next()
        {
            if !verb.starts_with('-') && !offered.iter().any(|held| return held == verb)
            {
                offered.push(verb.to_owned());
            }
        }
    }

    return offered;
}

#[test]
fn Test_Every_Verb_The_Binary_Dispatches_Should_Appear_In_Its_Own_Help()
{
    let help = Help();
    let dispatched = Dispatched();

    assert!(
        dispatched.len() >= 4,
        "only {dispatched:?} were found in the dispatch, so this guard covers almost nothing"
    );

    let offered = Offered(&help);
    assert!(
        !offered.is_empty(),
        "no usage line was found in the help, so this guard is reading the wrong thing"
    );

    let unmentioned: Vec<&String> = dispatched
        .iter()
        .filter(|verb| return !offered.iter().any(|shown| return shown == *verb))
        .collect();

    assert!(
        unmentioned.is_empty(),
        "the binary answers these and its own help offers no usage line for them, so a person \
         running the tool cannot find them: {unmentioned:?}"
    );
}

#[test]
fn Test_The_Help_Should_Not_Offer_A_Verb_The_Binary_Cannot_Answer()
{
    // The other direction. A usage line promising something the dispatch refuses sends a reader
    // to find out the hard way, which is what the README-to-dispatch check catches for the
    // README and nothing caught for the help.
    let help = Help();
    let dispatched = Dispatched();
    let offered = Offered(&help);

    assert!(
        !offered.is_empty(),
        "no usage line was found in the help, so this guard is reading the wrong thing"
    );

    let unanswerable: Vec<&String> = offered
        .iter()
        .filter(|verb| return !dispatched.iter().any(|known| return known == *verb))
        .collect();

    assert!(
        unanswerable.is_empty(),
        "the help offers these and the binary does not dispatch them: {unanswerable:?}"
    );
}

#[test]
fn Test_The_Help_Should_Not_Claim_This_Repository_Cannot_Read_A_Source()
{
    // It said *There is no extractor* until `KWB-67`, which `KWB-66` had made false and
    // `KWB-56` had already corrected in the README — so the two surfaces contradicted each
    // other, introduced by the session that had closed that defect five times.
    //
    // The assertion is on the false sentence rather than on the true one, deliberately: pinning
    // the wording that replaced it would fail on every honest rephrasing, which is how a guard
    // becomes something people route around.
    let help = Help();

    assert!(
        !help.contains("There is no extractor"),
        "the help says this repository has no extractor, and `kwb-extract` is in the workspace"
    );
    assert!(
        help.contains("--says"),
        "the help no longer says how a claim gets in, so this test is passing on a help that \
         stopped explaining the command"
    );
}
