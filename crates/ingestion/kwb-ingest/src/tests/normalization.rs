//! Stage two: the structure is entitled to the closure it computes.
//!
//! [`Normalize_Concepts`] groups on content identity and takes no predicate, so a caller cannot
//! hand it a relation that is not transitive. That is the `D18` door, closed by not existing
//! rather than by being guarded — and these tests assert the closure on names the prototype's own
//! counterexample would have fused.
//!
//! [`Normalize_Concepts`]: crate::Normalize_Concepts

use super::*;

/// **The test this stage exists for.**
///
/// These three names are the prototype's own counterexample. A variant relation answers true
/// for the acronym against each expansion and false between the expansions, so a structure
/// computing a transitive closure over it fuses all three. That is how `zero matrix` and
/// `zero mass` became one concept, and how 144 of 579 merges came to fuse ideas the rule
/// explicitly rejects.
///
/// This asserts the **structure**, not a predicate, which is the half `D18` shows testing
/// cannot skip.
#[test]
fn Test_Names_A_Variant_Relation_Would_Bridge_Should_Remain_Distinct()
{
    let offered = [
        Offered("z m", "an abbreviation in the text"),
        Offered("zero mass", "a particle with no rest mass"),
        Offered("zero matrix", "the additive identity of a matrix ring"),
    ];
    let normalized = Normalize_Concepts(Link_Concepts(&offered));

    assert_eq!(
        normalized.Concepts().len(),
        offered.len(),
        "a bridge term fused concepts the relation would itself reject"
    );
    assert_eq!(normalized.Merged(), 0);
}

/// The control that stops the test above from passing for the wrong reason.
///
/// A normalizer that merged *nothing* would satisfy the bridge test perfectly and be useless.
/// This is the other direction: names that genuinely are one concept must become one.
#[test]
fn Test_Repeated_Mentions_Of_One_Concept_Should_Become_One_Concept()
{
    let offered = [
        Offered("entropy", "It is non-decreasing."),
        Offered("entropy", "It has units of joules per kelvin."),
        Offered("entropy", "It is extensive."),
    ];
    let normalized = Normalize_Concepts(Link_Concepts(&offered));

    assert_eq!(
        normalized.Concepts().len(),
        1,
        "the stage merged nothing, so the bridge test above proves nothing"
    );
    assert_eq!(
        normalized.Claims_Held(),
        offered.len(),
        "folding three mentions into one concept must not drop the claims"
    );
}

#[test]
fn Test_The_Grouping_Relation_Should_Be_Transitive_Over_A_Chain()
{
    // Closure is what the stage computes, so the relation it computes over has to be
    // transitive. Equality is, and a three-link chain is the smallest case that would
    // expose a relation that is not.
    let offered = [
        Offered("BVH", "one"),
        Offered("BVH", "two"),
        Offered("BVH", "three"),
    ];
    let normalized = Normalize_Concepts(Link_Concepts(&offered));

    assert_eq!(normalized.Concepts().len(), 1);
    assert_eq!(
        normalized.Merged(),
        offered.len() - normalized.Concepts().len(),
        "a three-link chain must fold to one concept and say how many it absorbed"
    );
}

#[test]
fn Test_Case_Differences_Should_Not_Be_Grouped()
{
    // The recorded divergence from the prototype's LOWER() index, exercised at the stage
    // where folding it would do the damage.
    let offered = [Offered("BVH", "one"), Offered("bvh", "two")];
    let normalized = Normalize_Concepts(Link_Concepts(&offered));

    assert_eq!(
        normalized.Concepts().len(),
        offered.len(),
        "folding case would have made these one concept"
    );
}

#[test]
fn Test_Normalization_Should_Remove_No_Claim()
{
    let linked = Link_Concepts(&[
        Offered("entropy", "one"),
        Offered("entropy", "two"),
        Offered("enthalpy", "three"),
    ]);
    let before = linked.Claims().len();

    let normalized = Normalize_Concepts(linked);

    assert_eq!(
        normalized.Claims_Held(),
        before,
        "merging concepts dropped a claim, which is a destructive edit with nothing \
         authorising it"
    );
}
