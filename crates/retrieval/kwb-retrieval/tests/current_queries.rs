//! What the current world answers, in the unit that file owns.
//!
//! # Why this exists beside `read_only.rs`
//!
//! `tests/read_only.rs` drives these queries from ten directions and its unit is `read_only`,
//! which is no source file's stem — so, to a rule that reads the unit, `CurrentQueries` was a
//! surface nothing tested. This is the companion of `src/current_queries.rs`: same subject,
//! addressed.
//!
//! `Concepts_Matching` was not covered anywhere at all before this file. The rest is thinner
//! here than next door on purpose — the behaviour is already asserted, and repeating it at
//! length would be a second copy to keep in step. What is asserted here is what the *unit*
//! has to be true of, with the one query function that had no test of its own given the same
//! treatment as its sibling.

use kwb_domain::{Assertion, Claim, Concept, KnowledgeGraph, Scope, Standing, Versioned};
use kwb_retrieval::CurrentQueries;

/// Two concepts, a claim about one of them, and the assertion of that claim.
struct Corpus
{
    /// The graph.
    graph: KnowledgeGraph,

    /// The concept the claim is about — two words, so a one-word query is a real test.
    entropy: Concept,

    /// The claim, whose identity the neighbourhood is expected to reach.
    claim: Claim,
}

/// The corpus above, assembled.
fn Corpus() -> Corpus
{
    let entropy = Concept::Named("thermodynamic entropy");
    let enthalpy = Concept::Named("enthalpy");
    let claim = Claim::About(&entropy, "It is non-decreasing in an isolated system.");
    let assertion = Assertion::By("Callen 1985", &claim, Scope::Named("physical theory").expect("a named scope"));

    let graph = KnowledgeGraph::Empty()
        .With_Concept(Versioned::Asserted(entropy.clone()))
        .With_Concept(Versioned::Asserted(enthalpy))
        .With_Claim(Versioned::Asserted(claim.clone()))
        .With_Assertion(Versioned::Asserted(assertion));

    return Corpus { graph, entropy, claim };
}

/// The corpus with its concept merged into a successor, which is the only state in which the
/// current and historical counts differ.
///
/// The successor is *published* and not merely named, because a superseded concept is replaced
/// rather than deleted: a graph that closed one without admitting the other would hold three
/// versions of nothing current, and the counts below would be 1 and 3 for a reason that has
/// nothing to do with what these tests are about.
fn After_A_Merge(corpus: &Corpus) -> KnowledgeGraph
{
    let keeper = Concept::Named("entropy");

    return corpus
        .graph
        .With_Concept(Versioned::Asserted(keeper.clone()))
        .With_Concept(
            Versioned::Asserted(corpus.entropy.clone()).Closed(Standing::Superseded {
                by: keeper.Identity(),
                because: "the two names denote one concept".to_owned(),
            }),
        );
}

#[test]
fn Test_Over_Should_Ask_Of_The_Graph_It_Was_Handed()
{
    // `Over` is a constructor, so it has no behaviour of its own to assert — and what it can
    // get wrong is binding the wrong graph, or defaulting, and then answering about a world
    // nobody named. Both readings are here so a default cannot pass.
    let corpus = Corpus();

    assert_eq!(CurrentQueries::Over(&corpus.graph).Claims_Matching("isolated").len(), 1);
    assert!(
        CurrentQueries::Over(&KnowledgeGraph::Empty())
            .Claims_Matching("isolated")
            .is_empty(),
        "a query answered about a graph other than the one it was handed"
    );
}

#[test]
fn Test_Concept_Count_Should_Count_Only_Concepts_That_Are_Current()
{
    // The distinction the two query types exist for, in one number: after a merge the graph
    // holds three concepts and two of them are current — the successor and the bystander — so a
    // count that gave three would be the historical answer arriving under the current name.
    let corpus = Corpus();
    let merged = After_A_Merge(&corpus);

    assert_eq!(CurrentQueries::Over(&merged).Concept_Count(), 2);
}

#[test]
fn Test_Concepts_Matching_Should_Find_A_Concept_Whose_Name_Holds_Every_Word_Of_The_Query()
{
    let corpus = Corpus();

    let found = CurrentQueries::Over(&corpus.graph).Concepts_Matching("thermodynamic entropy");

    assert_eq!(found.len(), 1);
    assert_eq!(
        found.first().map(|concept| return concept.Identity()),
        Some(corpus.entropy.Identity())
    );
}

#[test]
fn Test_Concepts_Matching_Should_Refuse_A_Name_That_Holds_Only_One_Word_Of_The_Query()
{
    // The same rule the claims search follows, asserted on the concept search because the two
    // share `Has_All_Words` and a report of "found" against a half-matching name is the
    // false-positive shape that makes a search worth ignoring.
    let corpus = Corpus();

    assert!(
        CurrentQueries::Over(&corpus.graph)
            .Concepts_Matching("thermodynamic enthalpy")
            .is_empty(),
        "a concept whose name holds one word of the query was returned"
    );
}

#[test]
fn Test_Concepts_Matching_Should_Match_Nothing_Rather_Than_Everything_For_An_Empty_Query()
{
    // The trap this is the wrong way round to fall into: `Has_All_Words` answers `true` for an
    // empty word list, so the emptiness has to be refused *before* it — and each query function
    // that declines must be shown to decline, rather than the pair resting on one of them.
    let corpus = Corpus();

    assert!(CurrentQueries::Over(&corpus.graph).Concepts_Matching("").is_empty());
    assert!(CurrentQueries::Over(&corpus.graph).Concepts_Matching("   ").is_empty());
}

#[test]
fn Test_Claims_Matching_Should_Answer_Nothing_For_A_Query_Of_Only_Spaces()
{
    let corpus = Corpus();

    assert!(CurrentQueries::Over(&corpus.graph).Claims_Matching("   ").is_empty());
}

#[test]
fn Test_Neighbourhood_Of_Should_Reach_The_Claims_And_The_Citations_Of_One_Concept()
{
    let corpus = Corpus();

    let neighbourhood = CurrentQueries::Over(&corpus.graph)
        .Neighbourhood_Of(corpus.entropy.Identity())
        .expect("the concept was never closed, so it is current");

    assert_eq!(neighbourhood.concept.Identity(), corpus.entropy.Identity());
    assert_eq!(
        neighbourhood.claims.first().map(|claim| return claim.Identity()),
        Some(corpus.claim.Identity())
    );
    assert_eq!(
        neighbourhood.assertions.first().map(|assertion| return assertion.Claim()),
        Some(corpus.claim.Identity()),
        "the neighbourhood did not reach the citation of its own claim"
    );
}

#[test]
fn Test_Neighbourhood_Of_Should_Answer_Nothing_For_A_Concept_That_Is_Not_Current()
{
    let corpus = Corpus();
    let merged = After_A_Merge(&corpus);

    assert!(
        CurrentQueries::Over(&merged)
            .Neighbourhood_Of(corpus.entropy.Identity())
            .is_none(),
        "a merge loser is not a current concept, so the current world answers nothing about it \
         and the caller has to ask the other one"
    );
}
