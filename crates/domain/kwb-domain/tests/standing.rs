//! Where a concept stands — asserted function by function.
//!
//! # Why this file is here rather than in `src/tests.rs`
//!
//! This crate's tests used to be one file, `src/tests.rs`, and its stem names no source file. So
//! every function below was exercised and addressed by nothing: the unit `check-test-coverage`
//! reads is the test file's own stem, and `src/epistemic/standing.rs` is covered by a file called
//! `standing.rs` and by nothing else. A failure in `Because` arrived under the name of a test about
//! a merge.
//!
//! # What this type replaces
//!
//! The prototype carried the same fact in **two columns** — `Status = Deprecated` and
//! `ValidUntil` — and then needed six SQL `CHECK` constraints to stop a row saying both things
//! inconsistently. One enum, and there is no contradictory state to refuse. `Because` is the half
//! of `D17` a successor cannot supply: an audit needs to re-read *what belief authorised this*,
//! and re-reading the prototype's merge log is how 144 of 579 merges were found to have fused
//! ideas the rule explicitly rejected.

use kwb_domain::Concept;
use kwb_domain::Standing;

/// Why the retired concept in these fixtures was retired.
const RETIRED_BECAUSE: &str = "the concept was withdrawn by its author";

/// Why the superseded concept in these fixtures was merged.
const SUPERSEDED_BECAUSE: &str = "the two names denote one concept";

/// The concept the superseded standing points at.
fn A_Successor() -> Concept
{
    return Concept::Named("C++");
}

/// One standing of each kind, in the order the enum declares them.
///
/// Named rather than written inside a test because `Not current` and `merged into something` are
/// two facts, and every test here needs all three states to tell them apart.
fn Every_Standing(successor: &Concept) -> [Standing; 3]
{
    return [
        Standing::Asserted,
        Standing::Retired { because: RETIRED_BECAUSE.to_owned() },
        Standing::Superseded {
            by: successor.Identity(),
            because: SUPERSEDED_BECAUSE.to_owned(),
        },
    ];
}

/// A standing's name in the enum, for an assertion that reads as a sentence.
fn Name_Of(standing: &Standing) -> &'static str
{
    return match *standing
    {
        Standing::Asserted => "asserted",
        Standing::Retired { .. } => "retired",
        Standing::Superseded { .. } => "superseded",
    };
}

#[test]
fn Test_Is_Current_Should_Be_True_Of_Asserted_And_Of_Nothing_Else()
{
    let successor = A_Successor();
    let standings = Every_Standing(&successor);

    let current: Vec<&'static str> = standings
        .iter()
        .filter(|standing| return standing.Is_Current())
        .map(Name_Of)
        .collect();
    let closed: Vec<&'static str> = standings
        .iter()
        .filter(|standing| return !standing.Is_Current())
        .map(Name_Of)
        .collect();

    assert_eq!(current, ["asserted"], "a concept that is not asserted is counted as current");
    assert_eq!(
        closed,
        ["retired", "superseded"],
        "a closed concept is not merely absent from the current read; it says so, and both ways of \
         being closed say so"
    );
}

#[test]
fn Test_Because_Should_Name_What_Authorised_Closing_It()
{
    // The prototype's `ConceptMerge` log re-read is how 144 of 579 merges were found to have fused
    // ideas the rule rejected -- months later, on the live corpus. An audit needs something to
    // re-read, and a successor alone is not it.
    let successor = A_Successor();
    let standings = Every_Standing(&successor);

    let reasons: Vec<Option<&str>> = standings
        .iter()
        .map(|standing| return standing.Because())
        .collect();

    assert_eq!(
        reasons,
        [None, Some(RETIRED_BECAUSE), Some(SUPERSEDED_BECAUSE)],
        "a closure that records only the fact of closing cannot answer what belief authorised it, \
         and then no merge already on record stays falsifiable"
    );
}

#[test]
fn Test_Superseded_By_Should_Name_The_Successor_When_There_Is_One()
{
    // Distinct from `Is_Current` on purpose: *not current* and *merged into something* are
    // different facts, and a retired concept has the first without the second. Reading them as one
    // is `D19-B`, whose `merge-audit` resolved none of the merge log's identifiers, printed
    // *"nothing has been merged away"* and exited `0`.
    let successor = A_Successor();

    assert_eq!(
        Standing::Superseded {
            by: successor.Identity(),
            because: SUPERSEDED_BECAUSE.to_owned(),
        }
        .Superseded_By(),
        Some(successor.Identity()),
        "a merge loser cannot say what it was merged into, so the merge log resolves to nothing"
    );
    assert_eq!(Standing::Asserted.Superseded_By(), None);
    assert_eq!(
        Standing::Retired { because: RETIRED_BECAUSE.to_owned() }.Superseded_By(),
        None,
        "a retired concept claims a successor it does not have, so an audit would resolve it to \
         something nothing was ever merged into"
    );
}
