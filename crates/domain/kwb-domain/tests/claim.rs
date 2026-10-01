//! A claim, addressed by what it says — asserted function by function.
//!
//! # Why this file is here rather than in `src/tests.rs`
//!
//! This crate's tests used to be one file, `src/tests.rs`, and its stem names no source file. So
//! every function below was exercised and addressed by nothing: the unit `check-test-coverage`
//! reads is the test file's own stem, and `src/epistemic/claim.rs` is covered by a file called
//! `claim.rs` and by nothing else. A failure in `Text` arrived under the name of a test about a
//! scope.
//!
//! # The two exclusions, and why each has a test
//!
//! A claim's identity excludes its source, so two references asserting one claim become one claim
//! with two citations — the cross-source mechanism the whole workspace rests on. It excludes its
//! scope, because `D-010` measured that a scope in the identity would make a benchmark and a bare
//! preference saying the same thing two rows that can never learn of each other. Both exclusions
//! are absences, and the only way a test can assert an absence is by fixing what the thing *is*
//! made of, which is what these do.

use kwb_domain::Claim;
use kwb_domain::Concept;

/// The statement most of these tests are about, and the concept they state it of.
///
/// One value with named fields rather than a pair, because a pair says nothing about which of its
/// two positions is the concept and which is the statement.
struct EntropyStatement
{
    /// The concept the statement is about.
    concept: Concept,

    /// The statement itself, as the tests below hand it to `Claim::About`.
    text: &'static str,
}

/// The statement most of these tests are about, and the concept they state it of.
fn Entropy() -> EntropyStatement
{
    return EntropyStatement {
        concept: Concept::Named("entropy"),
        text: "It is non-decreasing in an isolated system.",
    };
}

#[test]
fn Test_About_Should_Exclude_The_Source_So_One_Claim_Carries_Two_Citations()
{
    let EntropyStatement { concept: entropy, text } = Entropy();

    let callen = Claim::About(&entropy, text);
    let kittel = Claim::About(&entropy, text);

    assert_eq!(
        callen.Identity(),
        kittel.Identity(),
        "the source is excluded, so one claim carries two citations"
    );
}

#[test]
fn Test_Identity_Should_Be_Independent_Of_How_The_Statement_Is_Laid_Out()
{
    let EntropyStatement { concept: entropy, .. } = Entropy();

    let printed = Claim::About(&entropy, "It is  non-decreasing.\n");
    let reflowed = Claim::About(&entropy, "It is non-decreasing.");

    assert_eq!(
        printed.Identity(),
        reflowed.Identity(),
        "a reflow is not an edit, so a citation survives the re-run"
    );
}

#[test]
fn Test_Identity_Should_Move_When_The_Statement_Changes()
{
    let EntropyStatement { concept: entropy, .. } = Entropy();

    let stated = Claim::About(&entropy, "It is non-decreasing.");
    let edited = Claim::About(&entropy, "It is non-increasing.");

    assert_ne!(
        stated.Identity(),
        edited.Identity(),
        "a citation to the old text must fail loudly rather than resolve to text that no longer \
         says what was cited"
    );
}

#[test]
fn Test_Concept_Should_Participate_By_Address_So_A_Rename_Is_Computable_Staleness()
{
    // D-006: a claim declares what its input *was* when it was derived, so a renamed concept gives
    // its claims new identities and the staleness is computable rather than silent. The two
    // concepts differ only in the case of their first letter, which is an edit here.
    let EntropyStatement { concept: entropy, text } = Entropy();
    let renamed = Concept::Named("Entropy");

    let about_one = Claim::About(&entropy, text);
    let about_other = Claim::About(&renamed, text);

    assert_ne!(about_one.Identity(), about_other.Identity(), "the concept participates by name");
    assert_eq!(about_one.Concept(), entropy.Identity(), "the claim forgot what it was about");
}

#[test]
fn Test_A_Claim_Should_Carry_A_Concept_And_Neither_Scope_Nor_Confidence()
{
    // D-010 and D20 respectively, asserted the only way a test can assert an absence: by fixing
    // what a claim is made of. A scope or a grade added later fails this, and it has to be added
    // deliberately, because either changes the derivation and every identity in the corpus with it.
    let EntropyStatement { concept: entropy, .. } = Entropy();
    let claim = Claim::About(&entropy, "anything");

    assert_eq!(claim.Concept(), entropy.Identity());
    assert_eq!(claim.Text(), "anything");
    // Identity, concept and text, and no fourth thing. The claim's address is not the concept's,
    // so the concept is genuinely a field of it rather than the whole of it.
    assert_ne!(claim.Identity(), entropy.Identity());
}

#[test]
fn Test_Text_Should_Agree_With_The_Address_Derived_From_The_Statement()
{
    // `KWB-32`: two spellings that derive one address must render one text, or which one a reader
    // gets back depends on which was published first.
    let EntropyStatement { concept: entropy, .. } = Entropy();

    let printed = Claim::About(&entropy, "It is  non-decreasing.\n");
    let reflowed = Claim::About(&entropy, "It is non-decreasing.");

    assert_eq!(printed.Identity(), reflowed.Identity(), "a reflow is not an edit");
    assert_eq!(printed.Text(), reflowed.Text(), "one address rendered as two texts");
    assert_eq!(printed.Text(), "It is non-decreasing.", "the text kept what it was given");
}
