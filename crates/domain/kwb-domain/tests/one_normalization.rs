//! One normalization over four text-carrying types, and the one fold it must not do.
//!
//! # Why this file is named for a property rather than a type
//!
//! Every other file here is named for the source file it covers, because the unit a test belongs to
//! is the stem of the file it sits in. These two properties are not about one type: they are about
//! all four of the types that keep text agreeing with each other, which is a claim no single type's
//! file can make. `tests/one_liveness.rs` is the same shape for the rule that is applied once.
//!
//! A stem that names no source file covers nothing, and that is correct — there is nothing here for
//! `check-test-coverage` to address. The alternative would have been to file each of these under one
//! of the four types, which would say the property belongs to that type and to nothing else.

use kwb_domain::Assertion;
use kwb_domain::Claim;
use kwb_domain::Concept;
use kwb_domain::Scope;

#[test]
fn Test_All_Four_Text_Carrying_Types_Should_Agree_About_What_They_Keep()
{
    // Two of the four normalized what they stored and two did not, and nothing decided that: a
    // `Concept` and a `Claim` reflowed their text, while an `Assertion`'s source and its scope kept
    // whatever they were given. So two spellings that derived one address could render two
    // different things, and which one a reader got back depended on which was published first.
    //
    // `kwb-model`'s `Normalize_Text` is the one rule now, and this asserts that all four go through
    // it rather than that any one of them does.
    let concept = Concept::Named("a  b");
    let claim = Claim::About(&concept, "p  q");
    let assertion = Assertion::By("S  1", &claim, Scope::Named("z  y").expect("a named scope"));

    assert_eq!(concept.Canonical_Name(), "a b");
    assert_eq!(claim.Text(), "p q");
    assert_eq!(assertion.Source(), "S 1");
    assert_eq!(assertion.Scope().Name(), "z y");
}

#[test]
fn Test_Normalization_Should_Still_Not_Fold_Case()
{
    // The one normalization that looks obviously helpful and is not. The prototype arbitrated name
    // collisions with a partial unique index over `LOWER("CanonicalName")`, so `Polish notation`
    // and `polish notation` were one concept -- a merge with no way back, applied to every name in
    // the corpus at once.
    let upper = Concept::Named("BVH");
    let lower = Concept::Named("bvh");

    assert_ne!(upper.Identity(), lower.Identity());
    assert_ne!(upper.Canonical_Name(), lower.Canonical_Name());
}
