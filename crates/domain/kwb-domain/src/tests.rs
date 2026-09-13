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
    let from_one_book = Concept::Named("entropy");
    let from_another = Concept::Named("entropy");

    assert_eq!(from_one_book.Identity(), from_another.Identity());
}

#[test]
fn Test_Concepts_Differing_Only_In_Case_Should_Be_Two_Concepts()
{
    // The recorded divergence from the prototype, which folded case in a LOWER() index.
    let upper = Concept::Named("BVH");
    let lower = Concept::Named("bvh");

    assert_ne!(
        upper.Identity(),
        lower.Identity(),
        "case is significant here; folding it is the merge with no way back"
    );
}

#[test]
fn Test_Two_References_Asserting_One_Claim_Should_Produce_One_Claim()
{
    let entropy = Concept::Named("entropy");
    let text = "Entropy is non-decreasing in an isolated system.";

    let callen = Claim::About(&entropy, text);
    let kittel = Claim::About(&entropy, text);

    assert_eq!(
        callen.Identity(),
        kittel.Identity(),
        "the source is excluded, so one claim carries two citations"
    );
}

#[test]
fn Test_A_Claim_Should_Depend_On_Its_Concepts_Identity_And_Not_Its_Name()
{
    let entropy = Concept::Named("entropy");
    let renamed = Concept::Named("Entropy");
    let text = "It is non-decreasing in an isolated system.";

    let about_one = Claim::About(&entropy, text);
    let about_other = Claim::About(&renamed, text);

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
    let entropy = Concept::Named("entropy");

    let stated = Claim::About(&entropy, "It is non-decreasing.");
    let edited = Claim::About(&entropy, "It is non-increasing.");

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
    let entropy = Concept::Named("entropy");

    let printed = Claim::About(&entropy, "It is  non-decreasing.\n");
    let reflowed = Claim::About(&entropy, "It is non-decreasing.");

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
    let entropy = Concept::Named("entropy");
    let claim = Claim::About(&entropy, "anything");

    assert_eq!(claim.Text(), "anything");
    assert_eq!(claim.Concept(), entropy.Identity());
    // Identity, concept and text. If a scope or a confidence is ever added to this type,
    // the derivation above changes and every identity in the corpus changes with it --
    // which is the cost that makes the decision worth having been recorded.
    assert_ne!(claim.Identity(), entropy.Identity());
}

// ---- KWB-23: an assertion is where a source and a scope attach ----

fn Entropy() -> (Concept, Claim)
{
    let concept = Concept::Named("entropy");
    let claim = Claim::About(&concept, "It is non-decreasing in an isolated system.");
    return (concept, claim);
}

#[test]
fn Test_Two_Sources_Asserting_One_Claim_Should_Be_Two_Assertions_Of_One_Claim()
{
    let (_, claim) = Entropy();
    let thermodynamics = Scope::Named("physical theory");

    let callen = Assertion::By("Callen 1985", &claim, thermodynamics.clone());
    let kittel = Assertion::By("Kittel 1980", &claim, thermodynamics);

    assert_ne!(callen.Identity(), kittel.Identity(), "two sources, two assertions");
    assert_eq!(
        callen.Claim(),
        kittel.Claim(),
        "and they meet at the claim, which is the whole cross-source mechanism"
    );
}

#[test]
fn Test_One_Source_Asserting_One_Claim_Twice_Should_Be_One_Assertion()
{
    let (_, claim) = Entropy();

    let once = Assertion::By("Callen 1985", &claim, Scope::Named("physical theory"));
    let twice = Assertion::By("Callen 1985", &claim, Scope::Named("physical theory"));

    assert_eq!(once.Identity(), twice.Identity(), "re-reading a book is not a second citation");
}

#[test]
fn Test_The_Same_Source_At_A_Different_Scope_Should_Be_A_Different_Assertion()
{
    let (_, claim) = Entropy();

    let broad = Assertion::By("Callen 1985", &claim, Scope::Named("universal mathematics"));
    let narrow = Assertion::By("Callen 1985", &claim, Scope::Named("this game workload"));

    assert_ne!(
        broad.Identity(),
        narrow.Identity(),
        "how far a source claims something reaches is part of what it asserted"
    );
    assert_eq!(broad.Claim(), narrow.Claim());
}

#[test]
fn Test_A_Preference_And_A_Measurement_Saying_One_Thing_Should_Meet_At_The_Claim()
{
    // The contamination D-010 exists to prevent, and the thing that makes preventing it
    // worthwhile: whether the two agree is the one question worth asking of the pair, and it
    // is only askable because the claim is shared.
    let concept = Concept::Named("static dispatch");
    let text = "Under workload W, target H, constraints C, it was preferred.";
    let claim = Claim::About(&concept, text);

    let measured = Assertion::By("benchmark run 41", &claim, Scope::Named("project evidence"));
    let preferred = Assertion::By("the user", &claim, Scope::Named("user preference"));

    assert_eq!(
        measured.Claim(),
        preferred.Claim(),
        "a scope in the claim would have made these two rows that never learn of each other"
    );
    assert_ne!(measured.Identity(), preferred.Identity(), "and they are still distinguishable");
}

#[test]
fn Test_An_Assertion_Should_Carry_No_Strength_Of_Any_Kind()
{
    // D-004 holds the epistemic-strength model as pending. The prototype shipped the
    // alternative in forty types: Confidence { get; set; } = 1.0. This fixes the absence --
    // adding a grade changes the derivation and every assertion identity in the corpus.
    let (_, claim) = Entropy();
    let assertion = Assertion::By("Callen 1985", &claim, Scope::Named("physical theory"));

    assert_eq!(assertion.Source(), "Callen 1985");
    assert_eq!(assertion.Scope().Name(), "physical theory");
    assert_eq!(assertion.Claim(), claim.Identity());
    assert_ne!(assertion.Identity(), claim.Identity());
}

#[test]
fn Test_An_Unstated_Scope_Should_Be_Recognisable_Rather_Than_Guessed()
{
    let (_, claim) = Entropy();
    let unstated = Scope::Named("   ");

    assert!(unstated.Is_Unstated());
    let assertion = Assertion::By("Callen 1985", &claim, unstated);
    assert!(
        assertion.Scope().Is_Unstated(),
        "a source that did not say how far it meant has not said the narrowest thing"
    );
}

#[test]
fn Test_A_Reflowed_Scope_Should_Not_Be_A_Different_Scope()
{
    assert_eq!(Scope::Named("physical  theory
"), Scope::Named("physical theory"));
    assert_ne!(
        Scope::Named("Physical theory"),
        Scope::Named("physical theory"),
        "case is significant here, as it is for a concept"
    );
}

// ---- KWB-32: an accessor never disagrees with the identity derived from the same input ----

#[test]
fn Test_Two_Spellings_That_Derive_One_Identity_Should_Render_One_Text()
{
    let spaced = Concept::Named("a  b");
    let tight = Concept::Named("a b");

    assert_eq!(spaced.Identity(), tight.Identity(), "a reflow is not an edit");
    assert_eq!(
        spaced.Canonical_Name(),
        tight.Canonical_Name(),
        "the identity says one concept, so the accessor must not say two different things --          otherwise what a reader gets back depends on which was published first"
    );
}

#[test]
fn Test_A_Claims_Text_Should_Agree_With_Its_Identity_The_Same_Way()
{
    let concept = Concept::Named("entropy");
    let printed = Claim::About(&concept, "It is  non-decreasing.
");
    let reflowed = Claim::About(&concept, "It is non-decreasing.");

    assert_eq!(printed.Identity(), reflowed.Identity());
    assert_eq!(printed.Text(), reflowed.Text());
    assert_eq!(printed.Text(), "It is non-decreasing.");
}

#[test]
fn Test_All_Four_Text_Carrying_Types_Should_Agree_About_What_They_Keep()
{
    // Two of the four normalized what they stored and two did not, and nothing decided that.
    let concept = Concept::Named("a  b");
    let claim = Claim::About(&concept, "p  q");
    let assertion = Assertion::By("S  1", &claim, Scope::Named("z  y"));

    assert_eq!(concept.Canonical_Name(), "a b");
    assert_eq!(claim.Text(), "p q");
    assert_eq!(assertion.Source(), "S 1");
    assert_eq!(assertion.Scope().Name(), "z y");
}

#[test]
fn Test_Normalization_Should_Still_Not_Fold_Case()
{
    // The one normalization that looks obviously helpful and is not.
    let upper = Concept::Named("BVH");
    let lower = Concept::Named("bvh");

    assert_ne!(upper.Identity(), lower.Identity());
    assert_ne!(upper.Canonical_Name(), lower.Canonical_Name());
}
