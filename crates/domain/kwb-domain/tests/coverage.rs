//! Whether a rule ran and what it found, asserted in the file named for the type.
//!
//! # Why this file is here rather than in `src/tests.rs`
//!
//! This crate's tests used to be one file, `src/tests.rs`, and its stem names no source file. So
//! every function below was exercised and addressed by nothing: the unit `check-test-coverage`
//! reads is the test file's own stem, and `src/epistemic/coverage.rs` is covered by a file called
//! `coverage.rs` and by nothing else. A change to what a `Barren` is would have arrived under the
//! name of a test about a scope.
//!
//! The properties are the ones that file already asserted. What changed is that each now sits in
//! the unit it is about and names the function it drives, so a failure arrives under the name of
//! the thing that broke.
//!
//! `D17` is what all of it defends: the absence of an extraction is not evidence against an
//! artefact. Every test here is about keeping *never ran* distinguishable from *ran and found
//! nothing*, which is the distinction the prototype's 1,367 `Barren` rows erased.

use core::num::NonZeroUsize;

use kwb_domain::Coverage;

/// A count that is known non-zero at the call site, so a test reads as what it is testing.
fn Nonzero_Count(value: usize) -> NonZeroUsize
{
    return NonZeroUsize::new(value).expect("the test supplied a non-zero count");
}

/// How much material the run examined in the tests below.
///
/// It is larger than any findings count here on purpose, because a `Barren` reports how much was
/// read: a run that read a lot and found nothing is a different claim from one that read almost
/// nothing, and only the count it carries tells the two apart.
const EXAMINED_MATERIAL: usize = 112;

/// How many findings the completed run in the outcomes table reported, which is what separates a
/// `Yielded` from a `Barren` over the same material.
const FINDINGS_FROM_THE_COMPLETED_RUN: usize = 3;

/// How much material the three-outcome fixture's run examined.
///
/// A fixture of its own rather than the incident's, and named apart from it so that the two are
/// never taken for one run.
const EXAMINED_MATERIAL_IN_THE_THREE_OUTCOME_FIXTURE: usize = 9;

/// How much material the yield-of-nothing fixture's run examined.
const EXAMINED_MATERIAL_IN_THE_BARREN_FIXTURE: usize = 40;

/// How many findings the derivation's run reported.
///
/// `Of_Run` computes the outcome from it, so the number is meant to appear on both sides of the
/// assertion that follows -- that the derivation carries it through unchanged is the property.
const FINDINGS_IN_THE_DERIVATION_FIXTURE: usize = 2;

/// The four outcomes, one of each, in the order a reader of [`Coverage`] meets them.
///
/// Named rather than written inside a test because the list is the subject several of the tests
/// below share, and a case added here is a case they all see.
fn Every_Outcome() -> [Coverage; 4]
{
    return [
        Coverage::Of_Run(FINDINGS_FROM_THE_COMPLETED_RUN, Nonzero_Count(EXAMINED_MATERIAL)),
        Coverage::Of_Run(0, Nonzero_Count(EXAMINED_MATERIAL)),
        Coverage::Skipped { because: "the source is a duplicate of one already admitted" },
        Coverage::Unmet { prerequisite: "the linking stage" },
    ];
}

/// The three outcomes that found nothing, which is three answers and not one.
///
/// The trap this exists for: a caller asking `Findings() == 0` instead of asking the question,
/// and deleting on the strength of it.
fn Three_Outcomes_None_Of_Which_Found_Anything() -> [Coverage; 3]
{
    return [
        Coverage::Of_Run(0, Nonzero_Count(EXAMINED_MATERIAL_IN_THE_THREE_OUTCOME_FIXTURE)),
        Coverage::Skipped { because: "out of scope for this run" },
        Coverage::Unmet { prerequisite: "the chunker" },
    ];
}

#[test]
fn Test_Has_Run_Should_Tell_A_Run_That_Found_Nothing_From_One_That_Never_Ran()
{
    let ran_and_found_nothing = Coverage::Of_Run(0, Nonzero_Count(EXAMINED_MATERIAL));
    let never_ran = Coverage::Unmet { prerequisite: "the linking stage" };

    assert_ne!(ran_and_found_nothing, never_ran);
    assert!(ran_and_found_nothing.Has_Run());
    assert!(
        !never_ran.Has_Run(),
        "a rule whose prerequisite was absent is recorded as one that ran, so the prototype's \
         1,367 Barren rows have a way back in"
    );
}

#[test]
fn Test_Is_Evidence_Of_Absence_Should_Be_True_Only_Of_A_Completed_Barren_Run()
{
    let evidence: Vec<&'static str> = Every_Outcome()
        .iter()
        .filter(|outcome| return outcome.Is_Evidence_Of_Absence())
        .map(|outcome| return outcome.Name())
        .collect();

    assert_eq!(
        evidence,
        ["barren"],
        "D17: the absence of an extraction is not evidence against an artefact"
    );
}

#[test]
fn Test_Findings_Should_Be_Zero_For_Three_Outcomes_And_That_Is_Not_One_Answer()
{
    let none_found = Three_Outcomes_None_Of_Which_Found_Anything();

    for outcome in none_found
    {
        assert_eq!(outcome.Findings(), 0);
    }
    assert_eq!(
        none_found
            .iter()
            .filter(|outcome| return outcome.Is_Evidence_Of_Absence())
            .count(),
        1,
        "three outcomes report no findings and only one of them means there are none, so a caller \
         asking the count is asking the wrong question"
    );
}

#[test]
fn Test_Of_Run_Should_Derive_The_Outcome_From_The_Counts_It_Was_Given()
{
    // D20: a computed property cannot fall out of step with the counts it describes, and leaves
    // no setter for a caller to forget.
    assert_eq!(
        Coverage::Of_Run(0, Nonzero_Count(1)),
        Coverage::Barren { examined: Nonzero_Count(1) }
    );
    assert_eq!(
        Coverage::Of_Run(FINDINGS_IN_THE_DERIVATION_FIXTURE, Nonzero_Count(1)),
        Coverage::Yielded { findings: Nonzero_Count(FINDINGS_IN_THE_DERIVATION_FIXTURE) }
    );
}

#[test]
fn Test_Of_Run_Should_Answer_Barren_When_A_Completed_Run_Found_Nothing()
{
    let nothing_found = Coverage::Of_Run(0, Nonzero_Count(EXAMINED_MATERIAL_IN_THE_BARREN_FIXTURE));

    assert!(
        !matches!(nothing_found, Coverage::Yielded { .. }),
        "Yielded carries a NonZeroUsize, so a yield of nothing is not a state this type has"
    );
}

#[test]
fn Test_Name_Should_Be_The_Short_Stable_Word_For_Each_Of_The_Four_Outcomes()
{
    let named: Vec<&'static str> = Every_Outcome()
        .iter()
        .map(|outcome| return outcome.Name())
        .collect();

    assert_eq!(
        named,
        ["yielded", "barren", "skipped", "unmet"],
        "a report or a journal names these, so two outcomes sharing a name would make the two \
         indistinguishable in the one place a person reads"
    );
}
