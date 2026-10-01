//! Anything the graph holds, together with where it stands — asserted function by function.
//!
//! # Why this file is here rather than in `src/tests.rs`
//!
//! This crate's tests used to be one file, `src/tests.rs`, and its stem names no source file. So
//! every function below was exercised and addressed by nothing: the unit `check-test-coverage`
//! reads is the test file's own stem, and `src/versioning/versioned.rs` is covered by a file called
//! `versioned.rs` and by nothing else. A failure in `Closed` arrived under the name of a test about
//! liveness.
//!
//! # One type for three things, and why
//!
//! Standing means the same thing for a concept, a claim and an assertion, so there is one type
//! rather than three, and a second copy of the idea would be a second place for it to drift. The
//! value is what is held; the standing is whether that is still true.

use kwb_domain::Concept;
use kwb_domain::Standing;
use kwb_domain::Versioned;

/// The concept these tests hold, which is the value under test rather than the subject.
fn Held_Concept() -> Concept
{
    return Concept::Named("entropy");
}

/// Why the fixture's standing is closed, where one is.
const BECAUSE: &str = "the concept was withdrawn by its author";

#[test]
fn Test_Asserted_Should_Hand_It_Back_As_It_Was()
{
    let concept = Held_Concept();
    let identity = concept.Identity();

    let held = Versioned::Asserted(concept);

    assert_eq!(
        held.Value().Identity(),
        identity,
        "a value put in as asserted came back as something else"
    );
}

#[test]
fn Test_Value_Should_Hand_Back_What_Is_Held()
{
    let held = Versioned::Asserted(Held_Concept());

    assert_eq!(held.Value().Canonical_Name(), "entropy");
}

#[test]
fn Test_Standing_Should_Say_Where_It_Stands()
{
    // `D20`: a value that reports the good case until somebody remembers to say otherwise is not
    // an unfinished feature, it is a false one. `Asserted` is a constructor with a name rather than
    // a `Default`, so there is no zero value for an unset field to land on.
    assert!(Versioned::Asserted(Held_Concept()).Standing().Is_Current());
    assert!(
        !Versioned::Asserted(Held_Concept())
            .Closed(Standing::Retired { because: BECAUSE.to_owned() })
            .Standing()
            .Is_Current(),
        "a closed value still reports itself as current"
    );
}

#[test]
fn Test_Closed_Should_Keep_The_Value_And_Change_Only_Where_It_Stands()
{
    // A closure is a second value, not an edit: `D17` says nothing is destroyed, and the thing a
    // supersession must not do is lose what the loser said. `EveryVersion` reads these; the current
    // read walks past them.
    let concept = Held_Concept();
    let identity = concept.Identity();

    let held = Versioned::Asserted(concept);
    let closed = held.Closed(Standing::Superseded {
        by: Concept::Named("C++").Identity(),
        because: BECAUSE.to_owned(),
    });

    assert_eq!(
        closed.Value().Identity(),
        identity,
        "closing a value replaced what was held, so the concept that was merged away is gone"
    );
    assert!(held.Standing().Is_Current(), "closing one value changed another");
    assert!(!closed.Standing().Is_Current());
    assert!(closed.Standing().Superseded_By().is_some());
}
