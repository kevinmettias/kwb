//! Publishing into a versioned graph, and the two reads it hands out.
//!
//! # Why this file is here rather than in `src/tests.rs`
//!
//! This crate's tests used to be one file, `src/tests.rs`, and its stem names no source file. So
//! every function below was exercised and addressed by nothing: the unit `check-test-coverage`
//! reads is the test file's own stem, and `src/graph/knowledge_graph.rs` is covered by a file
//! called `knowledge_graph.rs` and by nothing else. A failure in `With_Assertion` arrived under the
//! name of a test about replay.
//!
//! # What is asserted where
//!
//! Every function here is public, so it is asserted from outside the crate against the surface a
//! caller has. The five crate-visible functions — the three maps the reads walk, the concept-
//! currency question the claim and assertion reads compose with, and the one sorting rule — cannot
//! be reached from a file that compiles as its own package, and are asserted in `mod tests` beside
//! them in `src/graph/knowledge_graph.rs`.
//!
//! `D19-B` is what the two reads exist for, and `tests/one_liveness.rs` asserts it as a property of
//! the crate. This file asserts it function by function.

use kwb_domain::Assertion;
use kwb_domain::Claim;
use kwb_domain::Concept;
use kwb_domain::KnowledgeGraph;
use kwb_domain::Scope;
use kwb_domain::Standing;
use kwb_domain::Versioned;

/// Why the fixtures below close something.
const BECAUSE: &str = "the two names denote one concept";

/// A graph holding one concept, which is the smallest graph that has anything to say.
fn A_Graph_Holding_One_Concept() -> (KnowledgeGraph, Concept)
{
    let concept = Concept::Named("entropy");
    let graph = KnowledgeGraph::Empty().With_Concept(Versioned::Asserted(concept.clone()));
    return (graph, concept);
}

/// A graph holding one concept and one claim about it, which is what a claim needs already
/// published to be readable at all.
fn A_Graph_Holding_One_Claim() -> (KnowledgeGraph, Claim)
{
    let concept = Concept::Named("entropy");
    let claim = Claim::About(&concept, "It is non-decreasing in an isolated system.");

    let graph = KnowledgeGraph::Empty()
        .With_Concept(Versioned::Asserted(concept))
        .With_Claim(Versioned::Asserted(claim.clone()));

    return (graph, claim);
}

/// A graph holding a concept, the claim about it, and a source's assertion of that claim.
fn A_Graph_Holding_One_Assertion() -> (KnowledgeGraph, Assertion)
{
    let concept = Concept::Named("entropy");
    let claim = Claim::About(&concept, "It is non-decreasing in an isolated system.");
    let assertion = Assertion::By(
        "Callen 1985",
        &claim,
        Scope::Named("physical theory").expect("a named scope"),
    );

    let graph = KnowledgeGraph::Empty()
        .With_Concept(Versioned::Asserted(concept))
        .With_Claim(Versioned::Asserted(claim))
        .With_Assertion(Versioned::Asserted(assertion.clone()));

    return (graph, assertion);
}

#[test]
fn Test_Empty_Should_Hold_Nothing_And_Answer_With_Nothing()
{
    // The zero of the fold, and it has to be a real one: every read is a walk over maps that do not
    // exist yet, so an `Empty` that answered with an error or a placeholder would put a branch in
    // each of them.
    let empty = KnowledgeGraph::Empty();

    assert!(empty.Current().Concepts().is_empty());
    assert!(empty.Current().Claims().is_empty());
    assert!(empty.Current().Assertions().is_empty());
    assert!(empty.Every_Version().Concepts().is_empty());
    assert!(empty.Every_Version().Claims().is_empty());
    assert!(empty.Every_Version().Assertions().is_empty());
    assert!(empty.Every_Version().Merge_Losers().is_empty());
}

#[test]
fn Test_With_Concept_Should_Answer_The_Concept_It_Added()
{
    let (graph, concept) = A_Graph_Holding_One_Concept();
    let held = graph.Current().Concepts();

    assert_eq!(held.len(), 1, "a published concept was not published");
    assert_eq!(
        held.first().expect("one concept was published").Identity(),
        concept.Identity(),
        "the graph holds a concept other than the one it was handed"
    );
}

#[test]
fn Test_With_Claim_Should_Answer_The_Claim_It_Added()
{
    let (graph, claim) = A_Graph_Holding_One_Claim();
    let held = graph.Current().Claims();

    assert_eq!(held.len(), 1, "a published claim was not published");
    assert_eq!(held.first().expect("one claim was published").Identity(), claim.Identity());
    assert_eq!(
        held.first().expect("one claim was published").Concept(),
        claim.Concept(),
        "the claim was republished as a claim about something else"
    );
}

#[test]
fn Test_With_Assertion_Should_Answer_The_Assertion_It_Added()
{
    let (graph, assertion) = A_Graph_Holding_One_Assertion();
    let held = graph.Current().Assertions();

    assert_eq!(held.len(), 1, "a published assertion was not published");
    assert_eq!(held.first().expect("one assertion was published").Identity(), assertion.Identity());
    assert_eq!(
        held.first().expect("one assertion was published").Source(),
        "Callen 1985",
        "the assertion came back with a different source"
    );
}

#[test]
fn Test_Current_Should_Not_Reach_What_Was_Closed()
{
    // The finding that made this the interesting question: a concept retired on its own is not
    // current, and neither is one merged into another -- and `Every_Version` still holds both.
    let (graph, concept) = A_Graph_Holding_One_Concept();
    let successor = Concept::Named("C++");

    let after = graph
        .With_Concept(Versioned::Asserted(successor.clone()))
        .With_Concept(
            Versioned::Asserted(concept.clone()).Closed(Standing::Superseded {
                by: successor.Identity(),
                because: BECAUSE.to_owned(),
            }),
        );

    let current: Vec<_> = after.Current().Concepts().into_iter().map(Concept::Identity).collect();

    assert_eq!(current, [successor.Identity()], "a merge loser is still counted as current");
    assert_eq!(
        after.Every_Version().Concepts().len(),
        2,
        "closing a concept removed it, which is the destruction D17 refuses"
    );
}

#[test]
fn Test_Every_Version_Should_Reach_What_The_Current_Read_Does_Not()
{
    // `D19-B`: the prototype's answer was two interfaces and a discipline about which one you
    // depend on, and `merge-audit` resolved **none** of the merge log's identifiers against the
    // filtered view, printed *"nothing has been merged away"* and exited `0`.
    let (graph, concept) = A_Graph_Holding_One_Concept();
    let successor = Concept::Named("C++");

    let after = graph
        .With_Concept(Versioned::Asserted(successor))
        .With_Concept(
            Versioned::Asserted(concept.clone()).Closed(Standing::Superseded {
                by: Concept::Named("C++").Identity(),
                because: BECAUSE.to_owned(),
            }),
        );

    assert_eq!(
        after.Every_Version().Merge_Losers().len(),
        1,
        "the merge log cannot be answered from here, which is the query it exists for"
    );
    assert!(
        after
            .Every_Version()
            .Merge_Losers()
            .first()
            .is_some_and(|held| return held.Value().Identity() == concept.Identity()),
        "the wrong concept was reported as merged away"
    );
}
