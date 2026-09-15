//! A concept, addressed by what it is called — asserted function by function.
//!
//! # Why this file is here rather than in `src/tests.rs`
//!
//! This crate's tests used to be one file, `src/tests.rs`, and its stem names no source file. So
//! every function below was exercised and addressed by nothing: the unit `check-test-coverage`
//! reads is the test file's own stem, and `src/epistemic/concept.rs` is covered by a file called
//! `concept.rs` and by nothing else. A failure in `Canonical_Name` arrived under the name of a
//! test about the assertion that carried the concept.
//!
//! # The two divergences from the prototype these tests hold
//!
//! Case is significant — the prototype arbitrated name collisions with a partial unique index over
//! `LOWER("CanonicalName")`, so `Polish notation` and `polish notation` were one concept and here
//! they are two. And a whitespace reflow is not an edit, so two spellings differing only in
//! whitespace derive one address and must render one name.

use kwb_domain::Concept;

/// One concept under two spellings that differ only in whitespace, which is the pair a reflow is
/// supposed not to distinguish.
fn One_Concept_Reflowed() -> [Concept; 2]
{
    return [Concept::Named("a  b"), Concept::Named("a b")];
}

/// One name spelled two ways that differ in something other than whitespace: the case of a letter.
fn One_Name_In_Two_Cases() -> [Concept; 2]
{
    return [Concept::Named("BVH"), Concept::Named("bvh")];
}

#[test]
fn Test_Named_Should_Derive_One_Address_From_One_Name()
{
    let from_one_book = Concept::Named("entropy");
    let from_another = Concept::Named("entropy");

    assert_eq!(
        from_one_book.Identity(),
        from_another.Identity(),
        "two references naming one concept must be one concept, which is D-006's dependency edge"
    );
}

#[test]
fn Test_Two_References_Naming_One_Concept_Should_Produce_One_Identity()
{
    let [spaced, tight] = One_Concept_Reflowed();

    assert_eq!(spaced.Identity(), tight.Identity(), "a reflow is not an edit");
}

#[test]
fn Test_Identity_Should_Not_Fold_Case_Because_A_Changed_Capitalisation_Is_An_Edit()
{
    let [upper, lower] = One_Name_In_Two_Cases();

    assert_ne!(
        upper.Identity(),
        lower.Identity(),
        "case is significant here; folding it is the merge with no way back"
    );
}

#[test]
fn Test_Canonical_Name_Should_Never_Disagree_With_The_Address_Derived_From_It()
{
    // The identity says one concept, so the accessor must not say two different things -- otherwise
    // what a reader gets back depends on which was published first, and in a repository whose
    // premise is that content decides everything, insertion order deciding anything is the defect.
    let [spaced, tight] = One_Concept_Reflowed();

    assert_eq!(spaced.Identity(), tight.Identity(), "a reflow is not an edit");
    assert_eq!(
        spaced.Canonical_Name(),
        tight.Canonical_Name(),
        "one address rendered as two names"
    );
    assert_eq!(tight.Canonical_Name(), "a b", "the name kept what it was given");
}
