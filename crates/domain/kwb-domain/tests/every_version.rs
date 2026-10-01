//! The all-versions read, and the one query that needs it: what was merged away.
//!
//! # Why this file is here rather than in `src/tests.rs`
//!
//! This crate's tests used to be one file, `src/tests.rs`, and its stem names no source file. So
//! every function below was exercised and addressed by nothing: the unit `check-test-coverage`
//! reads is the test file's own stem, and `src/versioning/every_version.rs` is covered by a file
//! called `every_version.rs` and by nothing else. A failure in `Merge_Losers` arrived under the name
//! of a test about a scope.
//!
//! # Why this read exists at all
//!
//! `D19-B`. A global query filter rewrote every query, including ones written by someone who had
//! never heard of it, and `merge-audit` consequently resolved **none** of the merge log's
//! identifiers, printed *"nothing has been merged away"* and exited `0` — a false negative reported
//! as a clean run. The prototype's answer was two interfaces and a discipline about which one you
//! depend on. Here it is two types, and reaching a merge loser means naming the one whose name says
//! so, at the call site, where a reader can see it.
//!
//! `Of` is the reader's constructor and is `pub(crate)`; its test sits beside it in
//! `src/versioning/every_version.rs`.

use kwb_domain::Assertion;
use kwb_domain::Claim;
use kwb_domain::Concept;
use kwb_domain::KnowledgeGraph;
use kwb_domain::Scope;
use kwb_domain::Standing;
use kwb_domain::Versioned;

/// Why the fixtures below close something.
const BECAUSE: &str = "the two names denote one concept";

/// How many concepts a merge leaves behind: the one that was closed, and the one it was closed
/// into. This read reaches both, which is what separates it from the current one.
const LOSER_AND_SUCCESSOR: usize = 2;

/// The concept every fixture here is about.
fn Entropy() -> Concept
{
    return Concept::Named("entropy");
}

/// The claim every fixture here is about.
fn Its_Claim(concept: &Concept) -> Claim
{
    return Claim::About(concept, "It is non-decreasing in an isolated system.");
}

/// A source's assertion of that claim.
fn Callens_Assertion(claim: &Claim) -> Assertion
{
    return Assertion::By(
        "Callen 1985",
        claim,
        Scope::Named("physical theory").expect("a named scope"),
    );
}

/// A graph holding a concept, the claim about it and an assertion of that claim, and that concept.
///
/// One value with named fields rather than a pair, because a pair says nothing about which of its
/// two positions is the graph and which is the concept inside it.
struct PublishedKnowledge
{
    /// The graph the three publications went into.
    graph: KnowledgeGraph,

    /// The concept the graph holds, which the claim is about.
    concept: Concept,
}

/// A graph holding a concept, the claim about it, and an assertion of that claim.
fn Published_Knowledge() -> PublishedKnowledge
{
    let concept = Entropy();
    let claim = Its_Claim(&concept);
    let assertion = Callens_Assertion(&claim);

    let graph = KnowledgeGraph::Empty()
        .With_Concept(Versioned::Asserted(concept.clone()))
        .With_Claim(Versioned::Asserted(claim))
        .With_Assertion(Versioned::Asserted(assertion));

    return PublishedKnowledge { graph, concept };
}

#[test]
fn Test_Concepts_Should_Reach_A_Concept_That_Was_Closed()
{
    // The distinction between this read and the other one, as one assertion: two concepts were
    // published, one of them was closed, and both are here. A `Merge_Losers` query that resolved
    // nothing is what `D19-B` measured, and it is only askable from this read.
    let PublishedKnowledge { graph, concept } = Published_Knowledge();
    let successor = Concept::Named("C++");

    let after = graph
        .With_Concept(Versioned::Asserted(successor))
        .With_Concept(
            Versioned::Asserted(concept).Closed(Standing::Superseded {
                by: Concept::Named("C++").Identity(),
                because: BECAUSE.to_owned(),
            }),
        );

    assert_eq!(
        after.Every_Version().Concepts().len(),
        LOSER_AND_SUCCESSOR,
        "a concept that was closed is gone from the read that exists to report it, so the merge \
         log cannot be answered from anywhere"
    );
}

#[test]
fn Test_Claims_Should_Reach_A_Claim_About_A_Concept_That_Was_Closed()
{
    // The claim's own standing is untouched here; what changed is the concept's. Replay reconstructs
    // what happened rather than what is currently true, so a claim about a retired concept is still
    // a claim that was published -- and it is what replay needs to rebuild the graph in order.
    let PublishedKnowledge { graph, concept } = Published_Knowledge();

    let after = graph.With_Concept(
        Versioned::Asserted(concept).Closed(Standing::Retired {
            because: BECAUSE.to_owned(),
        }),
    );

    assert_eq!(
        after.Every_Version().Claims().len(),
        1,
        "a claim about a concept that is not current is gone from the all-versions read, so the \
         graph cannot be rebuilt from its own records"
    );
}

#[test]
fn Test_Assertions_Should_Reach_An_Assertion_Of_A_Claim_That_Was_Closed()
{
    let PublishedKnowledge { graph, concept } = Published_Knowledge();

    let after = graph.With_Claim(
        Versioned::Asserted(Its_Claim(&concept)).Closed(Standing::Retired {
            because: BECAUSE.to_owned(),
        }),
    );

    assert_eq!(
        after.Every_Version().Assertions().len(),
        1,
        "an assertion of a claim that is not current is gone from the all-versions read, so what \
         was published stops being recoverable the moment it stops being current"
    );
}

/// The fixture graph once one of its concepts was merged into a successor and another was retired.
///
/// The two closures are the pair the query below has to tell apart, and they are deliberately
/// published together: `entropy` was merged into `C++` and `free energy` was retired by its author,
/// and only the first of them is a merge loser.
fn A_Graph_With_A_Merge_Loser_Beside_A_Retired_Concept() -> KnowledgeGraph
{
    let PublishedKnowledge { graph, .. } = Published_Knowledge();
    let successor = Concept::Named("C++");
    let withdrawn = Concept::Named("free energy");

    return graph
        .With_Concept(Versioned::Asserted(successor.clone()))
        .With_Concept(
            Versioned::Asserted(Entropy()).Closed(Standing::Superseded {
                by: successor.Identity(),
                because: BECAUSE.to_owned(),
            }),
        )
        .With_Concept(
            Versioned::Asserted(withdrawn).Closed(Standing::Retired {
                because: BECAUSE.to_owned(),
            }),
        );
}

/// The canonical names the merge-loser query answers, in the order it answers them.
fn Merge_Loser_Names(graph: &KnowledgeGraph) -> Vec<&str>
{
    return graph
        .Every_Version()
        .Merge_Losers()
        .into_iter()
        .map(|held| return held.Value().Canonical_Name())
        .collect();
}

#[test]
fn Test_Merge_Losers_Should_Reach_Every_Concept_Closed_Against_A_Successor()
{
    // The query `merge-audit` needed. Two facts have to be told apart for it to answer: *not
    // current* and *merged into something* are different, and a retired concept has the first
    // without the second -- so a read that asked `Is_Current` would report a withdrawn concept as
    // merged away, which is a merge log with entries that never happened.
    let after = A_Graph_With_A_Merge_Loser_Beside_A_Retired_Concept();

    assert_eq!(
        Merge_Loser_Names(&after),
        ["entropy"],
        "the merge log reports a concept that was retired rather than merged, or misses one that \
         was merged, and either way an audit of it answers a question nobody asked"
    );
}
