//! Stage one: linking infers nothing.
//!
//! [`Link_Concepts`] attaches a claim to the concept its own extraction named and infers no
//! relation, so there is no algebra here for anything to have to check — which is the property
//! these tests assert, rather than what a typical linking run produces.
//!
//! [`Link_Concepts`]: crate::Link_Concepts

use super::*;

#[test]
fn Test_A_Claim_Should_Be_Linked_Only_To_The_Concept_Its_Own_Extraction_Named()
{
    let offered = [
        Offered("entropy", "It is non-decreasing in an isolated system."),
        Offered("enthalpy", "It is a thermodynamic potential."),
    ];
    let linked = Link_Concepts(&offered);

    assert_eq!(linked.Concepts().len(), offered.len(), "each extraction named its own concept");
    assert_eq!(linked.Claims().len(), offered.len(), "and each extraction became one claim");
    for (claim, concept) in linked.Claims().iter().zip(linked.Concepts())
    {
        assert_eq!(
            claim.Concept(),
            concept.Identity(),
            "a claim reached a concept its extraction did not name"
        );
    }
}

#[test]
fn Test_Every_Claims_Concept_Should_Be_Among_The_Concepts()
{
    let linked = Link_Concepts(&[
        Offered("entropy", "one"),
        Offered("enthalpy", "two"),
        Offered("entropy", "three"),
    ]);

    for claim in linked.Claims()
    {
        assert!(
            linked
                .Concepts()
                .iter()
                .any(|concept| return concept.Identity() == claim.Concept()),
            "a claim points at a concept this stage did not produce"
        );
    }
}

#[test]
fn Test_An_Incomplete_Extraction_Should_Be_Refused_And_Counted()
{
    let offered = [
        Offered("entropy", "It is non-decreasing."),
        Offered("enthalpy", "   "),
        Offered("", "orphaned text"),
    ];
    let linked = Link_Concepts(&offered);

    assert_eq!(linked.Claims().len(), 1);
    assert_eq!(
        linked.Claims().len() + linked.Refused(),
        offered.len(),
        "a stage that discards input without saying how much is why D19's reporting rule \
         exists"
    );
}
