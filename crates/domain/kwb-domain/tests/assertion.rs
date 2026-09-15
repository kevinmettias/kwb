//! One source asserting one claim at one scope — asserted function by function.
//!
//! # Why this file is here rather than in `src/tests.rs`
//!
//! This crate's tests used to be one file, `src/tests.rs`, and its stem names no source file. So
//! every function below was exercised and addressed by nothing: the unit `check-test-coverage`
//! reads is the test file's own stem, and `src/epistemic/assertion.rs` is covered by a file called
//! `assertion.rs` and by nothing else. A failure in `Source` arrived under the name of a test about
//! a scope.
//!
//! # What this type is for
//!
//! `Claim` excludes the source from its identity so that two references become one claim with two
//! citations, and excludes the scope because `D-010` measured that a scope in the identity would
//! make a benchmark and a bare preference two rows that can never learn of each other. Both
//! exclusions are right, and together they leave a claim with nowhere to record who said it or how
//! far they meant it — which is this type. **One claim, many assertions**, and the claim is where
//! they meet.

use kwb_domain::Assertion;
use kwb_domain::Claim;
use kwb_domain::Concept;
use kwb_domain::Scope;

/// A claim about entropy, which is the proposition every assertion below asserts.
///
/// The concept is not returned with it: each caller names only the claim, and a pair whose first
/// half nobody reads is a tuple asking to be destructured and discarded.
fn Entropy_Claim() -> Claim
{
    let concept = Concept::Named("entropy");
    return Claim::About(&concept, "It is non-decreasing in an isolated system.");
}

/// A scope a source stated, which is the one most of these assertions are made at.
fn Physical_Theory() -> Scope
{
    return Scope::Named("physical theory").expect("a named scope");
}

#[test]
fn Test_By_Should_Answer_The_Same_Address_When_It_Is_Given_The_Same_Three_Things()
{
    // All three participate in the identity, so re-recording the same source asserting the same
    // claim at the same scope is the same assertion. Re-reading a book is not a second citation.
    let claim = Entropy_Claim();

    let once = Assertion::By("Callen 1985", &claim, Physical_Theory());
    let twice = Assertion::By("Callen 1985", &claim, Physical_Theory());

    assert_eq!(once.Identity(), twice.Identity());
}

#[test]
fn Test_Identity_Should_Move_When_How_Far_It_Reaches_Moves()
{
    let claim = Entropy_Claim();

    let broad = Assertion::By("Callen 1985", &claim, Scope::Named("universal mathematics").expect("a named scope"));
    let narrow = Assertion::By("Callen 1985", &claim, Scope::Named("this game workload").expect("a named scope"));

    assert_ne!(
        broad.Identity(),
        narrow.Identity(),
        "how far a source claims something reaches is part of what it asserted"
    );
    assert_eq!(broad.Claim(), narrow.Claim(), "and it is not part of the claim");
}

#[test]
fn Test_Two_Sources_Asserting_One_Claim_Should_Be_Two_Assertions_Of_One_Claim()
{
    let claim = Entropy_Claim();

    let callen = Assertion::By("Callen 1985", &claim, Physical_Theory());
    let kittel = Assertion::By("Kittel 1980", &claim, Physical_Theory());

    assert_ne!(callen.Identity(), kittel.Identity(), "two sources, two assertions");
    assert_eq!(
        callen.Claim(),
        kittel.Claim(),
        "and they meet at the claim, which is the whole cross-source mechanism"
    );
}

#[test]
fn Test_A_Preference_And_A_Measurement_Should_Meet_At_The_Claim()
{
    // The contamination `D-010` exists to prevent, and the thing that makes preventing it
    // worthwhile: whether the two agree is the one question worth asking of the pair, and it is
    // only askable because the claim is shared.
    let concept = Concept::Named("static dispatch");
    let claim = Claim::About(&concept, "Under workload W, target H, constraints C, it was preferred.");

    let measured = Assertion::By("benchmark run 41", &claim, Scope::Named("project evidence").expect("a named scope"));
    let preferred = Assertion::By("the user", &claim, Scope::Named("user preference").expect("a named scope"));

    assert_eq!(
        measured.Claim(),
        preferred.Claim(),
        "a scope in the claim would have made these two rows that never learn of each other"
    );
    assert_ne!(measured.Identity(), preferred.Identity(), "and they are still distinguishable");
}

#[test]
fn Test_One_Source_Asserting_One_Claim_Twice_Should_Be_One_Assertion()
{
    let claim = Entropy_Claim();

    let once = Assertion::By("Callen 1985", &claim, Physical_Theory());
    let twice = Assertion::By("Callen 1985", &claim, Physical_Theory());

    assert_eq!(once.Identity(), twice.Identity(), "re-reading a book is not a second citation");
    assert_eq!(once.Source(), twice.Source());
}

#[test]
fn Test_Source_Should_Answer_Who_Asserted_It()
{
    // Normalized through `kwb-model`, like every other text this crate keeps: a reflow is not an
    // edit, so a source written with a doubled space is the same source.
    let claim = Entropy_Claim();

    assert_eq!(
        Assertion::By("Callen 1985", &claim, Physical_Theory()).Source(),
        "Callen 1985",
        "the source came back with something added or removed"
    );
    assert_eq!(
        Assertion::By("S  1", &claim, Physical_Theory()).Source(),
        "S 1",
        "a run of whitespace in a source's name was not collapsed"
    );
}

#[test]
fn Test_Scope_Should_Answer_How_Far_It_Reaches()
{
    // The distinction `KWB-50` closed: a source that did not say how far it meant has not said the
    // narrowest thing, and the two are told apart by asking rather than by guessing.
    let claim = Entropy_Claim();

    assert_eq!(
        Assertion::By("Callen 1985", &claim, Physical_Theory()).Scope().Name(),
        "physical theory"
    );
    assert!(
        Assertion::By("Callen 1985", &claim, Scope::Unstated()).Scope().Is_Unstated(),
        "an assertion by a source that stated no scope was given one on its behalf"
    );
}

#[test]
fn Test_An_Assertion_Should_Carry_No_Strength_Of_Any_Kind()
{
    // `D-004` holds the epistemic-strength model as pending, and the prototype shipped the
    // alternative in forty domain types: `Confidence { get; set; } = 1.0`, a settable field
    // defaulting to certainty. This fixes the absence. Adding a grade changes the derivation above
    // and every assertion identity in the corpus with it, which is the cost that makes the decision
    // worth having been recorded rather than drifted into.
    let claim = Entropy_Claim();
    let assertion = Assertion::By("Callen 1985", &claim, Physical_Theory());

    assert_eq!(assertion.Source(), "Callen 1985");
    assert_eq!(assertion.Scope().Name(), "physical theory");
    assert_eq!(assertion.Claim(), claim.Identity());
    assert_ne!(assertion.Identity(), claim.Identity());
}
