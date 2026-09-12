//! Tests for the properties these types exist to have.
//!
//! Named for the property rather than the function, because the defect each defends against
//! is a caller reaching a state another way — not a function returning a wrong value.

use core::num::NonZeroUsize;

use super::*;

/// A count that is known non-zero at the call site, so a test reads as what it is testing.
fn Count(value: usize) -> NonZeroUsize
{
    return NonZeroUsize::new(value).expect("the test supplied a non-zero count");
}

// ---- Coverage: the 1,367-row incident ----

#[test]
fn Test_Barren_And_Unmet_Should_Not_Be_Interchangeable()
{
    let ran_and_found_nothing = Coverage::Of_Run(0, Count(112));
    let never_ran = Coverage::Unmet { prerequisite: "the linking stage" };

    assert_ne!(ran_and_found_nothing, never_ran);
    assert!(ran_and_found_nothing.Was_Run());
    assert!(!never_ran.Was_Run());
    assert!(
        ran_and_found_nothing.Is_Evidence_Of_Absence() && !never_ran.Is_Evidence_Of_Absence(),
        "the prototype wrote 1,367 Barren rows meaning the prerequisite had not run, and \
         foreclosed two thirds of a book while reporting full coverage"
    );
}

#[test]
fn Test_Only_A_Completed_Run_That_Found_Nothing_Should_Be_Evidence_Of_Absence()
{
    let outcomes = [
        Coverage::Of_Run(3, Count(112)),
        Coverage::Of_Run(0, Count(112)),
        Coverage::Skipped { because: "the source is a duplicate of one already admitted" },
        Coverage::Unmet { prerequisite: "the linking stage" },
    ];

    let evidence: Vec<&'static str> = outcomes
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
fn Test_Three_Outcomes_Have_No_Findings_And_Only_One_Means_There_Are_None()
{
    // The trap this defends: a caller asking `Findings() == 0` instead of asking the
    // question, and deleting on the strength of it.
    let none_found = Coverage::Of_Run(0, Count(9));
    let skipped = Coverage::Skipped { because: "out of scope for this run" };
    let unmet = Coverage::Unmet { prerequisite: "the chunker" };

    for outcome in [none_found, skipped, unmet]
    {
        assert_eq!(outcome.Findings(), 0);
    }
    assert_eq!(
        [none_found, skipped, unmet]
            .iter()
            .filter(|outcome| return outcome.Is_Evidence_Of_Absence())
            .count(),
        1
    );
}

#[test]
fn Test_An_Outcome_Of_A_Run_Should_Be_Derived_From_What_It_Found()
{
    // D20: a computed property cannot fall out of step with the counts it describes, and
    // leaves no setter for a caller to forget.
    assert_eq!(Coverage::Of_Run(0, Count(1)), Coverage::Barren { examined: Count(1) });
    assert_eq!(
        Coverage::Of_Run(2, Count(1)),
        Coverage::Yielded { findings: Count(2) }
    );
}

#[test]
fn Test_A_Yield_Of_Nothing_Should_Be_A_Barren_Rather_Than_A_Yield_Of_Zero()
{
    let nothing_found = Coverage::Of_Run(0, Count(40));

    assert_eq!(nothing_found.Name(), "barren");
    assert!(
        !matches!(nothing_found, Coverage::Yielded { .. }),
        "Yielded carries a NonZeroUsize, so a yield of nothing is not a state this type has"
    );
}

// ---- Concept and Claim: identity, and the two exclusions ----

#[test]
fn Test_Two_References_Naming_One_Concept_Should_Produce_One_Identity()
{
    let from_one_book = Concept::Named("entropy".to_owned());
    let from_another = Concept::Named("entropy".to_owned());

    assert_eq!(from_one_book.Identity(), from_another.Identity());
}

#[test]
fn Test_Concepts_Differing_Only_In_Case_Should_Be_Two_Concepts()
{
    // The recorded divergence from the prototype, which folded case in a LOWER() index.
    let upper = Concept::Named("BVH".to_owned());
    let lower = Concept::Named("bvh".to_owned());

    assert_ne!(
        upper.Identity(),
        lower.Identity(),
        "case is significant here; folding it is the merge with no way back"
    );
}

#[test]
fn Test_Two_References_Asserting_One_Claim_Should_Produce_One_Claim()
{
    let entropy = Concept::Named("entropy".to_owned());
    let text = "Entropy is non-decreasing in an isolated system.";

    let callen = Claim::About(&entropy, text.to_owned());
    let kittel = Claim::About(&entropy, text.to_owned());

    assert_eq!(
        callen.Identity(),
        kittel.Identity(),
        "the source is excluded, so one claim carries two citations"
    );
}

#[test]
fn Test_A_Claim_Should_Depend_On_Its_Concepts_Identity_And_Not_Its_Name()
{
    let entropy = Concept::Named("entropy".to_owned());
    let renamed = Concept::Named("Entropy".to_owned());
    let text = "It is non-decreasing in an isolated system.";

    let about_one = Claim::About(&entropy, text.to_owned());
    let about_other = Claim::About(&renamed, text.to_owned());

    assert_ne!(
        about_one.Identity(),
        about_other.Identity(),
        "D-006: a claim declares what its input was, so a changed concept is computable \
         staleness rather than a silent reattachment"
    );
    assert_eq!(about_one.Concept(), entropy.Identity());
}

#[test]
fn Test_A_Changed_Claim_Should_Not_Keep_Its_Identity()
{
    let entropy = Concept::Named("entropy".to_owned());

    let stated = Claim::About(&entropy, "It is non-decreasing.".to_owned());
    let edited = Claim::About(&entropy, "It is non-increasing.".to_owned());

    assert_ne!(
        stated.Identity(),
        edited.Identity(),
        "a citation to the old text must fail loudly rather than resolve to text that no \
         longer says what was cited"
    );
}

#[test]
fn Test_A_Reflowed_Claim_Should_Keep_Its_Identity()
{
    let entropy = Concept::Named("entropy".to_owned());

    let printed = Claim::About(&entropy, "It is  non-decreasing.\n".to_owned());
    let reflowed = Claim::About(&entropy, "It is non-decreasing.".to_owned());

    assert_eq!(
        printed.Identity(),
        reflowed.Identity(),
        "a reflow is not an edit, so a citation survives the re-run"
    );
}

#[test]
fn Test_A_Claim_Should_Carry_No_Scope_And_No_Confidence()
{
    // D-010 and D20 respectively, asserted the only way a test can assert an absence:
    // by fixing what a claim is made of. A field added later fails this.
    let entropy = Concept::Named("entropy".to_owned());
    let claim = Claim::About(&entropy, "anything".to_owned());

    assert_eq!(claim.Text(), "anything");
    assert_eq!(claim.Concept(), entropy.Identity());
    // Identity, concept and text. If a scope or a confidence is ever added to this type,
    // the derivation above changes and every identity in the corpus changes with it --
    // which is the cost that makes the decision worth having been recorded.
    assert_ne!(claim.Identity(), entropy.Identity());
}
